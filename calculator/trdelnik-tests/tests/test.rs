use fehler::throws;
use program_client::{calculator_instruction};
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

impl Fixture {
    fn new() -> Self {
        Fixture {
            client: Client::new(system_keypair(0)),
            program: program_keypair(1),
            mycalculator: keypair(42), //why 42?
        }
    }

    // #[throws]
    // async fn deploy(&mut self) {
    //     self.client
    //         .airdrop(self.client.payer().pubkey(), 5_000_000_000)
    //         .await?;
    // }
    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
        self.client
            .deploy_by_name(&self.program.clone(), "BabyYoda")
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
    let mut fixture = Fixture {
        client: Client::new(system_keypair(0)),
        program: program_keypair(1),
        mycalculator: keypair(42), //why 42?
    };

        // deploy a tested program
    fixture
        .deploy()
        .await?;

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
async fn test_init(#[future] init_fixture: Result<Fixture>) {
    let default_fixture = Fixture::new();
    let fixture = init_fixture.await?;
    assert_eq!(fixture.program, default_fixture.program);
    assert_eq!(1,1);
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
