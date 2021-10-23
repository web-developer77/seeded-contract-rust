import {
  establishConnection,
  establishPayer,
  checkProgram,
  sayHello,
  reportGreetings,
} from './hello_world';

async function main() {
  console.log("Let's say hello to a Solana account...");

  await establishConnection(); // Establish connection to the cluster
  await establishPayer(); // Determine who pays for the fees
  await checkProgram(); // Check if the program has been deployed
  await sayHello(); // Say hello to an account
  await reportGreetings(); // Find out how many times that account has been greeted

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
