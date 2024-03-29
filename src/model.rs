/// Defining models for the code
///
use serde::{Deserialize, Serialize};
use std::string::String;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
/// variant types in the vcf file
pub enum VariantType {
    /// a single nucleotide variant
    SNV,
    /// insertion or deletion
    INDEL,
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
/// zygostiy of a variant
pub enum Zygosity {
    /// a homozygous variant
    HOMOZYGOUS,
    /// a heterozygous variant
    HETEROZYGOUS,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// A struct to hold the contamination hypothesis
pub struct Hypothesis {
    /// labeling the hypothesis (e.g. homozygous, het-alt)
    pub label: String,
    /// the expected variant allele fraction
    pub variant_fraction: f64,
    /// the loglikelihood of the hypothesis calculated from the observed data
    pub loglik: Option<f64>,
}

impl Hypothesis {
    /// Create a Hypothesis object
    ///
    /// # Arguments
    /// * `label`: the name of the hypothesis
    /// * `variant_fraction`: the hypothesized contamination level
    ///
    /// Example::
    ///
    /// ```
    /// use diploid_contam_estimator::model::{Hypothesis};
    /// let hyp = Hypothesis::new(
    ///     "homozygous".to_string(),
    ///     0.1,
    /// ).unwrap();
    /// ```
    pub fn new(label: String, variant_fraction: f64) -> Result<Hypothesis, String> {
        if (0.0..=1.0).contains(&variant_fraction) {
            Ok(Self {
                label,
                variant_fraction,
                loglik: None,
            })
        } else {
            Err("variant_fraction must be between 0 and 1".to_string())
        }
    }

    /// Adding the log likelihood value after evaluation
    ///
    /// # Arguments
    /// * `loglik`: log likelihood value
    ///
    /// Example::
    ///
    /// ```
    /// use diploid_contam_estimator::model::{Hypothesis};
    /// let mut hyp = Hypothesis::new(
    ///     "homozygous".to_string(),
    ///     0.1,
    /// ).unwrap();
    /// hyp.set_loglik(0.1);
    /// assert_eq!(0.1, hyp.loglik.unwrap());
    /// ```
    pub fn set_loglik(&mut self, loglik: f64) {
        self.loglik = Some(loglik);
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
/// A struct to hold the contamination estimation result
pub struct ContamProbResult {
    /// for the given contamination level
    pub contamination_level: f64,
    /// what is the log_likelihood given the observed variants?
    pub log_likelihood: f64,
}

#[derive(Serialize, Deserialize, Debug)]
/// data structure for  a variant position
pub struct VariantPosition {
    /// contig name for where the variant is located at
    pub contig: String,
    /// genomic position of the variant
    pub position: usize,
    /// total read depth at the variant position
    pub total_read_depth: usize,
    /// total read that showed alt alleles in the variant position
    pub alt_depth: usize,
    /// is it a indel or snv?
    pub variant_type: VariantType,
    /// the zygosity of the variant
    pub zygosity: Zygosity,
    /// the best hypothesis of the contamination source
    pub contamination_label: Option<String>,
}

impl VariantPosition {
    /// Create a VariantPosition object
    ///
    /// # Arguments
    /// * `contig`: chromosome name
    /// * `position`: genomic position on the chromosome
    /// * `total_read_depth`: total depth at the position
    /// * `alt_depth`: total read that showed alt alleles
    /// * `variant_type`: is it a indel or snv?
    /// * `zygosity`: the zygosity of the variant
    ///
    /// Example::
    ///
    /// ```
    /// use diploid_contam_estimator::model::{VariantPosition, Zygosity, VariantType};
    /// let variant = VariantPosition::new(
    ///     "chr1", 1, 100, 50, VariantType::SNV, Zygosity::HETEROZYGOUS
    /// );
    /// ```
    pub fn new(
        contig: &str,
        position: usize,
        total_read_depth: usize,
        alt_depth: usize,
        variant_type: VariantType,
        zygosity: Zygosity,
    ) -> Result<VariantPosition, &str> {
        if total_read_depth < alt_depth || total_read_depth < 1 {
            // validation of the input
            return Err("Total read depth should be >= alt depth and positive");
        }
        Ok(Self {
            contig: contig.to_string(),
            position,
            total_read_depth,
            alt_depth,
            variant_type,
            zygosity,
            contamination_label: None,
        })
    }

    pub fn set_contamination_label(&mut self, contamination_label: String) {
        self.contamination_label = Some(contamination_label);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Total read depth should be >= alt dept")]
    fn test_variant_position_exception() {
        let _vp = VariantPosition::new(
            "chrX",
            1,
            100,
            101,
            VariantType::SNV,
            Zygosity::HETEROZYGOUS,
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "and positive")]
    fn test_variant_position_exception_0_depth() {
        let depth: i32 = 0;
        let _vp = VariantPosition::new(
            "chrX",
            1,
            depth as usize,
            0,
            VariantType::SNV,
            Zygosity::HETEROZYGOUS,
        )
        .unwrap();
    }

    #[test]
    fn test_variant_position() {
        let depth: i32 = 200;
        let mut vp = VariantPosition::new(
            "chrX",
            1,
            depth as usize,
            101,
            VariantType::SNV,
            Zygosity::HETEROZYGOUS,
        )
        .unwrap();
        let contam_label = "contam".to_string();
        let _ = &vp.set_contamination_label(contam_label.clone());
        assert_eq!(vp.contamination_label.unwrap(), contam_label);
    }

    #[test]
    fn test_hypothesis() {
        let mut hyp = Hypothesis::new("test_hyp".to_string(), 0.1).unwrap();
        hyp.set_loglik(0.2);
        assert_eq!(hyp.loglik, Some(0.2));
    }
}
