# bam_links

`bam_links` scans paired-end BAM files to detect potential structural variation signals:

- Inter-chromosomal read pairs (e.g. chr1 → chr5)
- Long-range intra-chromosomal read pairs (e.g. >10kb span)

All detected reads can be written to a new BAM for visual inspection in IGV or downstream analysis.

---

## Features

- Fast sequential BAM scanning (rust-htslib backend)
- Inter-chromosomal link detection
- Long-range mate detection
- Writes filtered reads to BAM
- Optional duplicate filtering
- Progress bar
- Summary reporting

---

## Requirements

### System packages (Ubuntu / Debian)

These packages are required so that `htslib` can compile:

```bash
sudo apt-get update
sudo apt-get install -y   gcc   make   clang   pkg-config   libbz2-dev   zlib1g-dev   libncurses5-dev   libncursesw5-dev   liblzma-dev
```

---

## Installation

### 1. Install Rust

If you do not have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify:

```bash
rustc --version
cargo --version
```

---

### 2. Build bam_links

Clone the repository and build:

```bash
git clone https://github.com/YOURNAME/bam_links.git
cd bam_links
cargo build --release
```

The binary will be produced at:

```
target/release/bam_links
```

---

## Usage

```bash
bam_links --bam input.bam --out hits.bam
```

### Help screen

```bash
bam_links --help
```

Output:

```
Detect inter-chromosomal or long-range paired-end links

Usage: bam_links [OPTIONS] --bam <BAM> --out <OUT>

Options:
  -b, --bam <BAM>            Input BAM file (must be indexed)
  -m, --max-dist <MAX_DIST>  Distance threshold (default: 10000)
  -o, --out <OUT>            Output BAM with discordant pairs
      --min-mapq <MIN_MAPQ>  Minimum mapping quality
      --no-dups              Ignore duplicates
      --summary-only         Output only summary, no individual lines
  -h, --help                 Print help
```

---

## Example (real dataset)

```bash
./target/release/bam_links   --bam NA12878.mapped.ILLUMINA.bwa.CEU.high_coverage_pcr_free.20130906.bam   --out translocations.bam
```

---

## Output

The output BAM contains only reads that were classified as:

- inter-chromosomal
- or long-range mates

Index after writing:

```bash
samtools index translocations.bam
```

Load the file in IGV to inspect.

---

## Notes

- The input BAM must be indexed (`.bai` present).
- This tool is most meaningful for:
  - WGS / WES
  - cancer sequencing
  - SV benchmarking datasets
- It is **not** meaningful on 10x Genomics scRNA or ATAC BAMs.

---

## License

MIT
