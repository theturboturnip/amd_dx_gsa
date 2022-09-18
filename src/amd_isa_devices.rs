#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/device_bindings.rs"));

#[derive(Debug, Copy, Clone)]
pub struct Asic {
    pub name: &'static str,        // const char *name;
    pub gfxIpString: &'static str, // const char *gfxIpString;
    pub chipFamily: i32,           // int chipFamily;
    pub chipRevision: i32,         // int chipRevision;
    pub targetName: &'static str,  // const char *targetName;
}
impl Asic {
    const fn from_tuple(t: (&'static str, &'static str, i32, i32, &'static str)) -> Self {
        let (name, gfxIpString, chipFamily, chipRevision, targetName) = t;
        Asic {
            name,
            gfxIpString,
            chipFamily,
            chipRevision,
            targetName,
        }
    }
}

pub const LEGACY_ASIC_COUNT: usize = 20;
pub const ASIC_COUNT: usize = 31;

pub const FIRST_RDNA2_ASIC: Asic = Asic::from_tuple((
    "RDNA (gfx1010)",
    "1010",
    FAMILY_NV,
    NV_NAVI10_P_A0,
    "gfx1010",
));

pub const ASIC_INFO: [Asic; ASIC_COUNT] = [
    // Southern Islands
    Asic::from_tuple(("GCN (Tahiti)", "6", FAMILY_SI, SI_TAHITI_P_B1, "gfx600")),
    Asic::from_tuple((
        "GCN (Pitcairn)",
        "6",
        FAMILY_SI,
        SI_PITCAIRN_PM_A1,
        "gfx601",
    )),
    Asic::from_tuple((
        "GCN (Capeverde)",
        "6",
        FAMILY_SI,
        SI_CAPEVERDE_M_A1,
        "gfx601",
    )),
    Asic::from_tuple(("GCN (Oland)", "6", FAMILY_SI, SI_OLAND_M_A0, "gfx601")),
    Asic::from_tuple(("GCN (Hainan)", "6", FAMILY_SI, SI_HAINAN_V_A0, "gfx601")),
    // Sea Islands
    Asic::from_tuple(("GCN (Bonaire)", "7", FAMILY_CI, CI_BONAIRE_M_A0, "gfx704")),
    Asic::from_tuple(("GCN (Hawaii)", "7", FAMILY_CI, CI_HAWAII_P_A0, "gfx701")),
    Asic::from_tuple(("GCN (Spectre)", "7", FAMILY_CI, KV_SPECTRE_A0, "gfx700")),
    Asic::from_tuple(("GCN (Spooky)", "7", FAMILY_CI, KV_SPOOKY_A0, "gfx700")),
    Asic::from_tuple(("GCN (Kalindi)", "7.x", FAMILY_CI, CI_BONAIRE_M_A0, "gfx703")),
    Asic::from_tuple(("GCN (Mullins)", "7", FAMILY_CI, CI_BONAIRE_M_A0, "gfx704")),
    // Volcanic Islands
    Asic::from_tuple(("GCN (Iceland)", "8", FAMILY_VI, VI_ICELAND_M_A0, "gfx802")),
    Asic::from_tuple(("GCN (Tonga)", "8", FAMILY_VI, VI_TONGA_P_A0, "gfx802")),
    Asic::from_tuple(("GCN (Carrizo)", "8", FAMILY_VI, CARRIZO_A0, "gfx801")),
    Asic::from_tuple(("GCN (Bristol Ridge)", "8", FAMILY_VI, CARRIZO_A0, "gfx801")),
    Asic::from_tuple(("GCN (Carrizo)", "8", FAMILY_VI, CARRIZO_A0, "gfx801")),
    Asic::from_tuple(("GCN (Fiji)", "8", FAMILY_VI, VI_FIJI_P_A0, "gfx803")),
    Asic::from_tuple(("GCN (Stoney)", "8.1", FAMILY_VI, STONEY_A0, "gfx810")),
    Asic::from_tuple((
        "GCN (Ellesmere)",
        "8",
        FAMILY_VI,
        VI_ELLESMERE_P_A0,
        "gfx803",
    )),
    Asic::from_tuple(("GCN (Baffin)", "8", FAMILY_VI, VI_BAFFIN_M_A0, "gfx803")),
    Asic::from_tuple(("GCN (gfx804)", "8", FAMILY_VI, VI_LEXA_V_A0, "gfx804")),
    // Arctic Islands
    Asic::from_tuple(("GCN (gfx900)", "900", FAMILY_AI, AI_GD_P0, "gfx900")),
    Asic::from_tuple(("GCN (gfx902)", "902", FAMILY_AI, AI_GD_P0, "gfx902")),
    Asic::from_tuple(("GCN (gfx906)", "906", FAMILY_AI, AI_VEGA20_P_A0, "gfx906")),
    // Navi
    FIRST_RDNA2_ASIC,
    Asic::from_tuple((
        "RDNA (gfx1012)",
        "1012",
        FAMILY_NV,
        NV_NAVI14_M_A0,
        "gfx1012",
    )),
    Asic::from_tuple((
        "RDNA2 (gfx1030)",
        "1030",
        FAMILY_NV,
        NV_NAVI21_P_A0,
        "gfx1030",
    )),
    Asic::from_tuple((
        "RDNA2 (gfx1031)",
        "1031",
        FAMILY_NV,
        NV_NAVI22_P_A0,
        "gfx1031",
    )),
    Asic::from_tuple((
        "RDNA2 (gfx1032)",
        "1032",
        FAMILY_NV,
        NV_NAVI23_P_A0,
        "gfx1032",
    )),
    Asic::from_tuple((
        "RDNA2 (gfx1034)",
        "1034",
        FAMILY_NV,
        NV_NAVI24_P_A0,
        "gfx1034",
    )),
    Asic::from_tuple((
        "RDNA2 (gfx1035)",
        "1035",
        FAMILY_RMB,
        REMBRANDT_A0,
        "gfx1035",
    )),
];
