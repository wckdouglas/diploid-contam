/// Defining models for the code
///
use serde::{Deserialize, Serialize};
use std::string::String;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum VariantType {
    /// variant types in the vcf file
    SNV,
    INDEL,
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Zygosity {
    /// zygostiy of a variant
    HOMOZYGOUS,
    HETEROZYGOUS,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContamProbResult {
    pub contamination_level: f64,
    pub log_likelihood: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VariantPosition {
    /// data structure for  a variant position
    pub contig: String,
    pub position: i64,
    pub total_read_depth: usize, // total read depth at the variant position
    pub alt_depth: usize,        // total read that showed alt alleles in the variant position
    pub variant_type: VariantType, // is it a indel or snv?
    pub zygosity: Zygosity,      // the zygosity of the variant
}

impl VariantPosition {
    pub fn new(
        contig: &str,
        position: i64,
        total_read_depth: usize,
        alt_depth: usize,
        variant_type: VariantType,
        zygosity: Zygosity,
    ) -> Self {
        if total_read_depth < alt_depth {
            panic!("Total read depth should be >= alt depth")
        }
        Self {
            contig: contig.to_string(),
            position: position,
            total_read_depth: total_read_depth,
            alt_depth: alt_depth,
            variant_type: variant_type,
            zygosity: zygosity,
        }
    }
}