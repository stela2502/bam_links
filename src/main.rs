use clap::Parser;
use rust_htslib::bam::{self, Read};
use std::collections::HashSet;
use indicatif::{ProgressBar, ProgressStyle};
use rust_htslib::bam::Writer;
use rust_htslib::bam::Format;

#[derive(Parser, Debug)]
#[command(name="bam_links", about="Detect inter-chromosomal or long-range paired-end links")]
struct Args {
    /// Input BAM file (must be indexed)
    #[arg(short, long)]
    bam: String,

    /// Distance threshold for intra-chromosomal links (e.g. 10000 = 10kb)
    #[arg(short, long, default_value_t = 10_000)]
    max_dist: i64,

    /// Output BAM with discordant pairs
    #[arg(short, long)]
    out: String,

    /// Minimum mapping quality
    #[arg(long, default_value_t = 0)]
    min_mapq: u8,

    /// Ignore duplicates
    #[arg(long)]
    no_dups: bool,

    /// Output only summary, no individual lines
    #[arg(long)]
    summary_only: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut bam = bam::Reader::from_path(&args.bam)?;
    let header_view = bam.header().clone();  // for tid2name()
    let header = bam::Header::from_template(bam.header());

    let mut writer = Writer::from_path(
        &args.out,
        &header,
        Format::Bam,
    )?;

    let mut inter_count = 0usize;
    let mut long_count = 0usize;
    let mut seen = HashSet::new();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green}  {pos} reads processed  ({per_sec})")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠸", "⠴", "⠦", "⠇"]),
    );
    let mut processed: u64 = 0;

    for rec in bam.records() {
        let rec = rec?;
        processed += 1;
        pb.set_position(processed);
        // Skip unmapped or unpaired
        if !rec.is_paired() || rec.is_unmapped() {
            continue;
        }

        // Optional filters
        if rec.mapq() < args.min_mapq {
            continue;
        }

        if args.no_dups && rec.is_duplicate() {
            continue;
        }

        if rec.is_secondary() || rec.is_supplementary() {
            continue;
        }

        let tid = rec.tid();
        let mtid = rec.mtid();

        if mtid < 0 {
            continue;
        }

        let pos = rec.pos() as i64;
        let mpos = rec.mpos() as i64;

        let is_inter = tid != mtid;
        let is_long = !is_inter && (pos - mpos).abs() > args.max_dist;

        if !is_inter && !is_long {
            continue;
        }

        let qname = String::from_utf8_lossy(rec.qname()).to_string();
        if !seen.insert(qname.clone()) {
            continue; // already reported
        }

        if is_inter {
            inter_count += 1;
        } else {
            long_count += 1;
        }

        if !args.summary_only {
            let chr1 = String::from_utf8_lossy(header_view.tid2name(tid as u32));
            let chr2 = String::from_utf8_lossy(header_view.tid2name(mtid as u32));

            writer.write(&rec)?;

            println!(
                "{}\t{}:{}\t{}:{}\t{}",
                qname,
                chr1,
                pos,
                chr2,
                mpos,
                if is_inter { "INTERCHR" } else { "LONGRANGE" }
            );
        }
    }
    pb.finish_with_message("Done.");

    println!("\n===== SUMMARY =====");
    println!("Inter-chromosomal pairs: {}", inter_count);
    println!("Long-range pairs (> {} bp): {}", args.max_dist, long_count);
    println!("Total discordant pairs: {}", inter_count + long_count);

    Ok(())
}

