// // <my_project>/trdelnik-tests/tests/test.rs
// // TODO: do not forget to add all necessary dependencies to the generated `trdelnik-tests/Cargo.toml`
// use program_client::calculator_instruction;
// use trdelnik_client::{anyhow::Result, *};
// use fehler::throws;
// use calculator;

// pub struct Fixture {
//     client: Client,
//     program: Keypair,
//     state: Keypair,
//   }


// #[throws]
// #[fixture]
// async fn init_fixture() -> Fixture {
//   // create a test fixture
//   let mut fixture = Fixture {
//     client: Client::new(system_keypair(0)),
//     // make sure your program is using a correct program ID
//     program: program_keypair(1),
//     state: keypair(42),
//   };
//   // deploy a tested program
//   fixture
//     .client
//     .deploy_by_name(&fixture.program, "turnstile")
//     .await?;

//   // call instruction init
//   calculator_instruction::create(
//     &fixture.client,
//     String::from("Calculator is on!"),
//     fixture.state.pubkey(),
//     fixture.client.payer().pubkey(),
//     System::id(),
//     Some(fixture.state.clone()),
//   ).await?;
//   fixture
// }


// impl Fixture {
//     #[throws]
//     async fn get_state(&self) -> calculator::Calculator {
//         self.client.account_data(self.state.pubkey()).await?
//     }
// }



// #[trdelnik_test]
// async fn test_happy_path(#[future] init_fixture: Result<Fixture, Error>) {
//   let fixture = init_fixture.await?;

//   let calc = fixture.get_state().await?;
//   assert_eq!(calc.result, 0);
// }