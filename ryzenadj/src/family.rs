use ryzenadj_sys as sys;

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
    Unknown(i32),
}

impl RyzenFamily {
    pub(crate) fn from_raw(raw: sys::ryzen_family) -> Self {
        match raw {
            sys::ryzen_family_FAM_RAVEN => Self::Raven,
            sys::ryzen_family_FAM_PICASSO => Self::Picasso,
            sys::ryzen_family_FAM_RENOIR => Self::Renoir,
            sys::ryzen_family_FAM_CEZANNE => Self::Cezanne,
            sys::ryzen_family_FAM_DALI => Self::Dali,
            sys::ryzen_family_FAM_LUCIENNE => Self::Lucienne,
            sys::ryzen_family_FAM_VANGOGH => Self::VanGogh,
            sys::ryzen_family_FAM_REMBRANDT => Self::Rembrandt,
            sys::ryzen_family_FAM_MENDOCINO => Self::Mendocino,
            sys::ryzen_family_FAM_PHOENIX => Self::Phoenix,
            sys::ryzen_family_FAM_HAWKPOINT => Self::HawkPoint,
            sys::ryzen_family_FAM_DRAGONRANGE => Self::DragonRange,
            sys::ryzen_family_FAM_KRACKANPOINT => Self::KrackanPoint,
            sys::ryzen_family_FAM_STRIXPOINT => Self::StrixPoint,
            sys::ryzen_family_FAM_STRIXHALO => Self::StrixHalo,
            sys::ryzen_family_FAM_FIRERANGE => Self::FireRange,
            raw => Self::Unknown(raw),
        }
    }
}
