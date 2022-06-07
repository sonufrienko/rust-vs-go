function main() {
  const count = 10_000_000;
  for (let i = 0; i <= count; i++) {
    const jsonStr =
      '{"first_name": "sergii","last_name": "onufriienko","year": 2022,"skills": ["node.js", "go", "rust", "aws", "k8s"],"happy": true,"text": "heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text"}';
    const jsonObj = JSON.parse(jsonStr);
  }
}

main();

// console.log('Done');
// setTimeout(() => {
//   console.log('End');
// }, 60000);
