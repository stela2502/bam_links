 curl -O ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/phase3/data/NA12878/high_coverage_alignment/NA12878.mapped.ILLUMINA.bwa.CEU.high_coverage_pcr_free.20130906.bam

 samtools view -h NA12878.mapped.ILLUMINA.bwa.CEU.high_coverage_pcr_free.20130906.bam | head -n 1000000 | samtools view -b - > positive_control_real.bam
 echo "you can remove the large NA12878.mapped.ILLUMINA.bwa.CEU.high_coverage_pcr_free.20130906.bam  now"
