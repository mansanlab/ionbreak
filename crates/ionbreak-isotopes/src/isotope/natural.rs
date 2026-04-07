use crate::element::Element;
use super::*;

// all naturally occuring isotopes sorted by most abundant first
static  H_ISOTOPES: [&Atom;  2] = [&H1   , &H2];
static HE_ISOTOPES: [&Atom;  2] = [&HE4  , &HE3];
static LI_ISOTOPES: [&Atom;  2] = [&LI7  , &LI6];
static BE_ISOTOPES: [&Atom;  1] = [&BE9];
static  B_ISOTOPES: [&Atom;  2] = [&B11  , &B10];
static  C_ISOTOPES: [&Atom;  2] = [&C12  , &C13];
static  N_ISOTOPES: [&Atom;  2] = [&N14  , &N15];
static  O_ISOTOPES: [&Atom;  3] = [&O16  , &O18  , &O17];
static  F_ISOTOPES: [&Atom;  1] = [&F19];
static NE_ISOTOPES: [&Atom;  3] = [&NE20 , &NE22 , &NE21];
static NA_ISOTOPES: [&Atom;  1] = [&NA23];
static MG_ISOTOPES: [&Atom;  3] = [&MG24 , &MG26 , &MG25];
static AL_ISOTOPES: [&Atom;  1] = [&AL27];
static SI_ISOTOPES: [&Atom;  3] = [&SI28 , &SI29 , &SI30];
static  P_ISOTOPES: [&Atom;  1] = [&P31];
static  S_ISOTOPES: [&Atom;  4] = [&S32  , &S34  , &S33  , &S36];
static CL_ISOTOPES: [&Atom;  2] = [&CL35 , &CL37];
static AR_ISOTOPES: [&Atom;  3] = [&AR40 , &AR36 , &AR38];
static  K_ISOTOPES: [&Atom;  3] = [&K39  , &K41  , &K40];
static CA_ISOTOPES: [&Atom;  6] = [&CA40 , &CA44 , &CA42 , &CA48 , &CA43 , &CA46];
static SC_ISOTOPES: [&Atom;  1] = [&SC45];
static TI_ISOTOPES: [&Atom;  5] = [&TI48 , &TI46 , &TI47 , &TI49 , &TI50];
static  V_ISOTOPES: [&Atom;  2] = [&V51  , &V50];
static CR_ISOTOPES: [&Atom;  4] = [&CR52 , &CR53 , &CR50 , &CR54];
static MN_ISOTOPES: [&Atom;  1] = [&MN55];
static FE_ISOTOPES: [&Atom;  4] = [&FE56 , &FE54 , &FE57 , &FE58];
static CO_ISOTOPES: [&Atom;  1] = [&CO59];
static NI_ISOTOPES: [&Atom;  5] = [&NI58 , &NI60 , &NI62 , &NI61 , &NI64];
static CU_ISOTOPES: [&Atom;  2] = [&CU63 , &CU65];
static ZN_ISOTOPES: [&Atom;  5] = [&ZN64 , &ZN66 , &ZN68 , &ZN67 , &ZN70];
static GA_ISOTOPES: [&Atom;  2] = [&GA69 , &GA71];
static GE_ISOTOPES: [&Atom;  5] = [&GE74 , &GE72 , &GE70 , &GE73 , &GE76];
static AS_ISOTOPES: [&Atom;  1] = [&AS75];
static SE_ISOTOPES: [&Atom;  6] = [&SE80 , &SE78 , &SE76 , &SE82 , &SE77 , &SE74];
static BR_ISOTOPES: [&Atom;  2] = [&BR79 , &BR81];
static KR_ISOTOPES: [&Atom;  6] = [&KR84 , &KR86 , &KR82 , &KR83 , &KR80 , &KR78];
static RB_ISOTOPES: [&Atom;  2] = [&RB85 , &RB87];
static SR_ISOTOPES: [&Atom;  4] = [&SR88 , &SR86 , &SR87 , &SR84];
static  Y_ISOTOPES: [&Atom;  1] = [&Y89];
static ZR_ISOTOPES: [&Atom;  5] = [&ZR90 , &ZR94 , &ZR92 , &ZR91 , &ZR96];
static NB_ISOTOPES: [&Atom;  1] = [&NB93];
static MO_ISOTOPES: [&Atom;  7] = [&MO98 , &MO96 , &MO95 , &MO92 , &MO100, &MO97 , &MO94];
static RU_ISOTOPES: [&Atom;  7] = [&RU102, &RU104, &RU101, &RU99 , &RU100, &RU96 , &RU98];
static RH_ISOTOPES: [&Atom;  1] = [&RH103];
static PD_ISOTOPES: [&Atom;  6] = [&PD106, &PD108, &PD105, &PD110, &PD104, &PD102];
static AG_ISOTOPES: [&Atom;  2] = [&AG107, &AG109];
static CD_ISOTOPES: [&Atom;  8] = [&CD114, &CD112, &CD111, &CD110, &CD113, &CD116, &CD106, &CD108];
static IN_ISOTOPES: [&Atom;  2] = [&IN115, &IN113];
static SN_ISOTOPES: [&Atom; 10] = [&SN120, &SN118, &SN116, &SN119, &SN117, &SN124, &SN122, &SN112, &SN114, &SN115];
static SB_ISOTOPES: [&Atom;  2] = [&SB121, &SB123];
static TE_ISOTOPES: [&Atom;  8] = [&TE130, &TE128, &TE126, &TE125, &TE124, &TE122, &TE123, &TE120];
static  I_ISOTOPES: [&Atom;  1] = [&I127];
static XE_ISOTOPES: [&Atom;  9] = [&XE132, &XE129, &XE131, &XE134, &XE136, &XE130, &XE128, &XE124, &XE126];
static CS_ISOTOPES: [&Atom;  1] = [&CS133];
static BA_ISOTOPES: [&Atom;  7] = [&BA138, &BA137, &BA136, &BA135, &BA134, &BA130, &BA132];
static LA_ISOTOPES: [&Atom;  2] = [&LA139, &LA138];
static CE_ISOTOPES: [&Atom;  4] = [&CE140, &CE142, &CE138, &CE136];
static PR_ISOTOPES: [&Atom;  1] = [&PR141];
static ND_ISOTOPES: [&Atom;  7] = [&ND142, &ND144, &ND146, &ND143, &ND145, &ND148, &ND150];
static SM_ISOTOPES: [&Atom;  7] = [&SM152, &SM154, &SM147, &SM149, &SM148, &SM150, &SM144];
static EU_ISOTOPES: [&Atom;  2] = [&EU153, &EU151];
static GD_ISOTOPES: [&Atom;  7] = [&GD158, &GD160, &GD156, &GD157, &GD155, &GD154, &GD152];
static TB_ISOTOPES: [&Atom;  1] = [&TB159];
static DY_ISOTOPES: [&Atom;  7] = [&DY164, &DY162, &DY163, &DY161, &DY160, &DY158, &DY156];
static HO_ISOTOPES: [&Atom;  1] = [&HO165];
static ER_ISOTOPES: [&Atom;  6] = [&ER166, &ER168, &ER167, &ER170, &ER164, &ER162];
static TM_ISOTOPES: [&Atom;  1] = [&TM169];
static YB_ISOTOPES: [&Atom;  7] = [&YB174, &YB172, &YB173, &YB171, &YB176, &YB170, &YB168];
static LU_ISOTOPES: [&Atom;  2] = [&LU175, &LU176];
static HF_ISOTOPES: [&Atom;  6] = [&HF180, &HF178, &HF177, &HF179, &HF176, &HF174];
static TA_ISOTOPES: [&Atom;  2] = [&TA181, &TA180];
static  W_ISOTOPES: [&Atom;  5] = [&W184 , &W186 , &W182 , &W183 , &W180];
static RE_ISOTOPES: [&Atom;  2] = [&RE187, &RE185];
static OS_ISOTOPES: [&Atom;  7] = [&OS192, &OS190, &OS189, &OS188, &OS187, &OS186, &OS184];
static IR_ISOTOPES: [&Atom;  2] = [&IR193, &IR191];
static PT_ISOTOPES: [&Atom;  6] = [&PT195, &PT194, &PT196, &PT198, &PT192, &PT190];
static AU_ISOTOPES: [&Atom;  1] = [&AU197];
static HG_ISOTOPES: [&Atom;  7] = [&HG202, &HG200, &HG199, &HG201, &HG198, &HG204, &HG196];
static TL_ISOTOPES: [&Atom;  2] = [&TL205, &TL203];
static PB_ISOTOPES: [&Atom;  4] = [&PB208, &PB206, &PB207, &PB204];
static BI_ISOTOPES: [&Atom;  1] = [&BI209];
static TH_ISOTOPES: [&Atom;  1] = [&TH232];
static PA_ISOTOPES: [&Atom;  1] = [&PA231];
static  U_ISOTOPES: [&Atom;  3] = [&U238 , &U235 , &U234];

pub static H : Element = Element::new( &H_ISOTOPES);
pub static HE: Element = Element::new(&HE_ISOTOPES);
pub static LI: Element = Element::new(&LI_ISOTOPES);
pub static BE: Element = Element::new(&BE_ISOTOPES);
pub static B : Element = Element::new( &B_ISOTOPES);
pub static C : Element = Element::new( &C_ISOTOPES);
pub static N : Element = Element::new( &N_ISOTOPES);
pub static O : Element = Element::new( &O_ISOTOPES);
pub static F : Element = Element::new( &F_ISOTOPES);
pub static NE: Element = Element::new(&NE_ISOTOPES);
pub static NA: Element = Element::new(&NA_ISOTOPES);
pub static MG: Element = Element::new(&MG_ISOTOPES);
pub static AL: Element = Element::new(&AL_ISOTOPES);
pub static SI: Element = Element::new(&SI_ISOTOPES);
pub static P : Element = Element::new( &P_ISOTOPES);
pub static S : Element = Element::new( &S_ISOTOPES);
pub static CL: Element = Element::new(&CL_ISOTOPES);
pub static AR: Element = Element::new(&AR_ISOTOPES);
pub static K : Element = Element::new( &K_ISOTOPES);
pub static CA: Element = Element::new(&CA_ISOTOPES);
pub static SC: Element = Element::new(&SC_ISOTOPES);
pub static TI: Element = Element::new(&TI_ISOTOPES);
pub static V : Element = Element::new( &V_ISOTOPES);
pub static CR: Element = Element::new(&CR_ISOTOPES);
pub static MN: Element = Element::new(&MN_ISOTOPES);
pub static FE: Element = Element::new(&FE_ISOTOPES);
pub static CO: Element = Element::new(&CO_ISOTOPES);
pub static NI: Element = Element::new(&NI_ISOTOPES);
pub static CU: Element = Element::new(&CU_ISOTOPES);
pub static ZN: Element = Element::new(&ZN_ISOTOPES);
pub static GA: Element = Element::new(&GA_ISOTOPES);
pub static GE: Element = Element::new(&GE_ISOTOPES);
pub static AS: Element = Element::new(&AS_ISOTOPES);
pub static SE: Element = Element::new(&SE_ISOTOPES);
pub static BR: Element = Element::new(&BR_ISOTOPES);
pub static KR: Element = Element::new(&KR_ISOTOPES);
pub static RB: Element = Element::new(&RB_ISOTOPES);
pub static SR: Element = Element::new(&SR_ISOTOPES);
pub static Y : Element = Element::new( &Y_ISOTOPES);
pub static ZR: Element = Element::new(&ZR_ISOTOPES);
pub static NB: Element = Element::new(&NB_ISOTOPES);
pub static MO: Element = Element::new(&MO_ISOTOPES);
pub static RU: Element = Element::new(&RU_ISOTOPES);
pub static RH: Element = Element::new(&RH_ISOTOPES);
pub static PD: Element = Element::new(&PD_ISOTOPES);
pub static AG: Element = Element::new(&AG_ISOTOPES);
pub static CD: Element = Element::new(&CD_ISOTOPES);
pub static IN: Element = Element::new(&IN_ISOTOPES);
pub static SN: Element = Element::new(&SN_ISOTOPES);
pub static SB: Element = Element::new(&SB_ISOTOPES);
pub static TE: Element = Element::new(&TE_ISOTOPES);
pub static I : Element = Element::new( &I_ISOTOPES);
pub static XE: Element = Element::new(&XE_ISOTOPES);
pub static CS: Element = Element::new(&CS_ISOTOPES);
pub static BA: Element = Element::new(&BA_ISOTOPES);
pub static LA: Element = Element::new(&LA_ISOTOPES);
pub static CE: Element = Element::new(&CE_ISOTOPES);
pub static PR: Element = Element::new(&PR_ISOTOPES);
pub static ND: Element = Element::new(&ND_ISOTOPES);
pub static SM: Element = Element::new(&SM_ISOTOPES);
pub static EU: Element = Element::new(&EU_ISOTOPES);
pub static GD: Element = Element::new(&GD_ISOTOPES);
pub static TB: Element = Element::new(&TB_ISOTOPES);
pub static DY: Element = Element::new(&DY_ISOTOPES);
pub static HO: Element = Element::new(&HO_ISOTOPES);
pub static ER: Element = Element::new(&ER_ISOTOPES);
pub static TM: Element = Element::new(&TM_ISOTOPES);
pub static YB: Element = Element::new(&YB_ISOTOPES);
pub static LU: Element = Element::new(&LU_ISOTOPES);
pub static HF: Element = Element::new(&HF_ISOTOPES);
pub static TA: Element = Element::new(&TA_ISOTOPES);
pub static W : Element = Element::new( &W_ISOTOPES);
pub static RE: Element = Element::new(&RE_ISOTOPES);
pub static OS: Element = Element::new(&OS_ISOTOPES);
pub static IR: Element = Element::new(&IR_ISOTOPES);
pub static PT: Element = Element::new(&PT_ISOTOPES);
pub static AU: Element = Element::new(&AU_ISOTOPES);
pub static HG: Element = Element::new(&HG_ISOTOPES);
pub static TL: Element = Element::new(&TL_ISOTOPES);
pub static PB: Element = Element::new(&PB_ISOTOPES);
pub static BI: Element = Element::new(&BI_ISOTOPES);
pub static TH: Element = Element::new(&TH_ISOTOPES);
pub static PA: Element = Element::new(&PA_ISOTOPES);
pub static U : Element = Element::new( &U_ISOTOPES);

#[cfg(test)]
mod tests {
    use super::*;

    pub static ALL_ELEMENTS: &[&Element] = &[
        &H , &HE, &LI, &BE, &B ,
        &C , &N , &O , &F , &NE, 
        &NA, &MG, &AL, &SI, &P ,
        &S , &CL, &AR, &K , &CA,
        &SC, &TI, &V , &CR, &MN,
        &FE, &CO, &NI, &CU, &ZN,
        &GA, &GE, &AS, &SE, &BR,
        &KR, &RB, &SR, &Y , &ZR,
        &NB, &MO, &RU, &RH, &PD,
        &AG, &CD, &IN, &SN, &SB,
        &TE, &I , &XE, &CS, &BA,
        &LA, &CE, &PR, &ND, &SM,
        &EU, &GD, &TB, &DY, &HO,
        &ER, &TM, &YB, &LU, &HF,
        &TA, &W , &RE, &OS, &IR,
        &PT, &AU, &HG, &TL, &PB,
        &BI, &TH, &PA, &U ,
    ];

    #[test]
    fn test_element_probs_sum_to_one() {
        for element in ALL_ELEMENTS {
            let sum: f64 = element.atoms.iter().map(|a| a.prob).sum();
            assert!(
                (sum - 1.0).abs() < 1e-15,
                "Element {} isotope probs sum to {} instead of 1.0",
                element.num(),
                sum
            );
        }
    }
}