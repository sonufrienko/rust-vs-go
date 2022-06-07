function main() {
  const count = 20_000_000;
  let mem = [];
  for (let i = 0; i <= count; i++) {
    mem.push({
      first_name: 'sergii',
      last_name: 'onufriienko',
      year: i,
      skills: ['node.js', 'go', 'rust', 'aws', 'k8s'],
      happy: true,
      text: 'heppy text',
    });
  }
}

main();

// console.log('Done');
// setTimeout(() => {
//   console.log('End');
// }, 60000);
