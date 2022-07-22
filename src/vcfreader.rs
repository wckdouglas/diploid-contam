use crate::model::{VariantPosition, VariantType, Zygosity};
use crate::rust_htslib::bcf::record::FilterId;
use crate::rust_htslib::bcf::{Read, Reader, Record};
use std::str::from_utf8;
use std::vec::Vec;

use log::info;

/// Colelcting variants from a vcf file
///
/// Arguments:
/// - vcf_file: file path to the vcf file we want to parse
/// - snv_only_flag: boolean flag indicating whether we shopuld only look at SNV instead of both SNV and indel
/// - depth_threshold: we will skip any variants with DP tag lower than this threshold
///
/// Returns:
/// - a list of varaints that passed the given filters
pub fn build_variant_list(
    vcf_file: &str,
    snv_only_flag: bool,
    depth_threshold: usize,
) -> Vec<VariantPosition> {
    let mut vcf: Reader = Reader::from_path(vcf_file).expect("Error opening file.");

    let mut variants: Vec<VariantPosition> = Vec::new();
    for (_i, record_result) in vcf.records().enumerate() {
        let record: Record = record_result.expect("Fail to read record");

        if record.filters().map(|filter| filter.is_pass()).all(|x| !!x) {
            // only look at pass filter variants

            let read_depth = record
                .format(b"DP")
                .integer()
                .ok()
                .expect("Error reading DP field.")[0][0] as usize;

            if read_depth >= depth_threshold {
                let allele_depth = record
                    .format(b"AD")
                    .integer()
                    .ok()
                    .expect("Error reading AD field.");
                let gts = record.genotypes().expect("Error reading genotypes");
                // assume theres only one sample in the vcf file hence:  get(0)
                // and diploid call (2nd genotype is non-ref), hence: [1].index
                let alt_call = gts.get(0)[1].index().unwrap() as usize;

                let mut variant_type: VariantType = VariantType::INDEL;
                if record.alleles()[0].len() == record.alleles()[1].len() {
                    // this should be testing the len of Vec<u8> where
                    // each item represents a base
                    // only if they are the same length, it's a SNV
                    variant_type = VariantType::SNV;
                }

                let mut zygosity = Zygosity::HOMOZYGOUS;
                if gts.get(0)[0] != gts.get(0)[1] {
                    // if the first genotype != the second genotype
                    // e.g. 0/1, 0/2
                    // then it's a heterozygous
                    zygosity = Zygosity::HETEROZYGOUS;
                }

                if !snv_only_flag || (snv_only_flag && variant_type == VariantType::SNV) {
                    // whether we want snv-only or not
                    // make a new VariantPosition here and put into the list
                    variants.push(VariantPosition::new(
                        from_utf8(record.header().rid2name(record.rid().unwrap()).unwrap())
                            .unwrap(),
                        record.pos(),
                        read_depth, // only sample in the vcf
                        allele_depth[0][alt_call] as usize,
                        variant_type, // TODO: fix this
                        zygosity,
                    ));
                }
            }
        }
    }
    info!("Collected {} variants from {}", variants.len(), vcf_file);
    return variants;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(false, 0, 14)] // all variants
    #[case(true, 0, 7)] // all SNV
    #[case(true, 1000, 6)] // all high depth SNV
    #[case(true, 1100, 1)] // all high depth SNV
    #[case(true, 1200, 0)] // all high depth SNV
    fn test_build_variant_list(
        #[case] snv_only_flag: bool,
        #[case] depth_threshold: usize,
        #[case] expected_number_variants: usize,
    ) {
        let vcf_file = "data/test.vcf";
        let variant_list = build_variant_list(&vcf_file, snv_only_flag, depth_threshold);
        assert_eq!(variant_list.len(), expected_number_variants);
    }
}
