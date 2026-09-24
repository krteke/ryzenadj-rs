use ryzenadj_sys as sys;

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RyzenFamily {
    Raven,
    Picasso,
    Renoir,
    Cezanne,
    Dali,
    Lucienne,
    VanGogh,
    Rembrandt,
    Mendocino,
    Phoenix,
    HawkPoint,
    DragonRange,
    KrackanPoint,
    StrixPoint,
    StrixHalo,
    FireRange,
}

impl TryFrom<sys::ryzen_family> for RyzenFamily {
    type Error = Error;

    fn try_from(raw: sys::ryzen_family) -> Result<Self> {
        match raw {
            sys::ryzen_family_FAM_RAVEN => Ok(Self::Raven),
            sys::ryzen_family_FAM_PICASSO => Ok(Self::Picasso),
            sys::ryzen_family_FAM_RENOIR => Ok(Self::Renoir),
            sys::ryzen_family_FAM_CEZANNE => Ok(Self::Cezanne),
            sys::ryzen_family_FAM_DALI => Ok(Self::Dali),
            sys::ryzen_family_FAM_LUCIENNE => Ok(Self::Lucienne),
            sys::ryzen_family_FAM_VANGOGH => Ok(Self::VanGogh),
            sys::ryzen_family_FAM_REMBRANDT => Ok(Self::Rembrandt),
            sys::ryzen_family_FAM_MENDOCINO => Ok(Self::Mendocino),
            sys::ryzen_family_FAM_PHOENIX => Ok(Self::Phoenix),
            sys::ryzen_family_FAM_HAWKPOINT => Ok(Self::HawkPoint),
            sys::ryzen_family_FAM_DRAGONRANGE => Ok(Self::DragonRange),
            sys::ryzen_family_FAM_KRACKANPOINT => Ok(Self::KrackanPoint),
            sys::ryzen_family_FAM_STRIXPOINT => Ok(Self::StrixPoint),
            sys::ryzen_family_FAM_STRIXHALO => Ok(Self::StrixHalo),
            sys::ryzen_family_FAM_FIRERANGE => Ok(Self::FireRange),
            sys::ryzen_family_FAM_UNKNOWN => Err(Error::UnsupportedFamily),
            code => Err(Error::InvalidFamily(code)),
        }
    }
}
