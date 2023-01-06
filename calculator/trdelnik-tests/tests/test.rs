use fehler::throws;
use program_client::calculator_instruction;
use trdelnik_client::{anyhow::Result, *};
use calculator;


// ------------------------------------- //
//           Fixture struct              //
// ------------------------------------- //
struct Fixture {
    client: Client,
    program: Keypair,
    mycalculator: Keypair,
}

const PROGRAM_KEYPAIRS: [[u8; 64]; 2] = [
    [
        249,58,221,247,231,9,219,29,233,175,32,213,16,176,192,13,71,23,79,52,49,61,101,219,131,
        188,245,160,19,176,236,156,89,162,119,162,57,107,154,54,221,229,215,68,151,213,56,78,
        229,124,124,203,80,38,222,57,128,18,80,254,185,58,41,118,
    ],
    [
        61, 157, 195, 135, 80, 255, 3, 105, 232, 208, 28, 48, 69, 104, 225, 202, 144, 183, 0, 123,
        108, 57, 165, 199, 168, 154, 194, 115, 18, 233, 99, 174, 5, 215, 176, 66, 255, 47, 77, 122,
        100, 249, 156, 251, 44, 92, 36, 220, 226, 147, 127, 109, 198, 92, 1, 127, 95, 116, 186,
        180, 149, 157, 170, 34,
    ]];

impl Fixture {
    pub fn gen_keypair(n: usize) -> Keypair {
        Keypair::from_bytes(&PROGRAM_KEYPAIRS[n]).unwrap()
    }

    fn new() -> Self {
        Fixture {
            client: Client::new(system_keypair(0)),
            program: Self::gen_keypair(0),
            // program: program_keypair(1),
            mycalculator: keypair(42),
        }
    }

    #[throws]
    async fn dep(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
    }

    #[throws]
    async fn get_calc(&self) -> calculator::Calculator {
        self.client.account_data(self.mycalculator.pubkey()).await?
    }
}


// ------------------------------------- //
//             Init Fixture              //
// ------------------------------------- //
#[throws]
#[fixture]
async fn init_fixture() -> Fixture {

    // create a test fixture
    // let fixture = Fixture::new();
    let mut fixture = Fixture::new();
    // deploy a tested program
    fixture.dep().await?;

    // init instruction call
    calculator_instruction::create(
        &fixture.client,
        String::from("Calculator is on!"),
        fixture.mycalculator.pubkey(),
        fixture.client.payer().pubkey(),
        System::id(),
        Some(fixture.mycalculator.clone()),
    )
    .await?;

    fixture
}


// ------------------------------------- //
//              Unit tests               //
// ------------------------------------- //
#[trdelnik_test]
async fn test_basic(#[future] init_fixture: Result<Fixture>) {
    let default_fixture = Fixture::new();
    let fixture = init_fixture.await?;
    assert_eq!(fixture.program, default_fixture.program);
}

#[trdelnik_test]
async fn test_nine() {
    let myvar = 9;
    assert_eq!(myvar, 9u8);
}

#[trdelnik_test]
#[ignore]
async fn test_multiply(#[future] init_fixture: Result<Fixture>) {
    // let default_fixture = Fixture::new();
    let fixture = init_fixture.await?;

    calculator_instruction::multiply(
        &fixture.client,
        3i64,
        3i64,
        fixture.mycalculator.pubkey(),
        None // since it's a PDA it needs no signature
    )
    .await?;

    // check the test result
    let mycalc = fixture.get_calc().await?;
    // 3x3 = 9 is true
    assert_eq!(mycalc.result, 9);
}