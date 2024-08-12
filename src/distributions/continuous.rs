pub mod beta;
pub mod bivariate_normal;
pub mod chi_squared;
pub mod exponential;
pub mod gamma;
pub mod log_normal;
pub mod student;
pub mod weibull;

pub use beta::BetaDistribution;
pub use bivariate_normal::BivariateNormalDistribution;
pub use chi_squared::ChiSquaredDistribution;
pub use exponential::ExponentialDistribution;
pub use gamma::GammaDistribution;
pub use log_normal::LogNormalDistribution;
pub use student::TDistribution;
pub use weibull::WeibullDistribution;
