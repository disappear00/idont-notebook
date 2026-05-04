const fs = require('fs');
const path = require('path');

const bins = Object.keys(require('../package.json').bin || {});
if (bins.length === 0) {
  console.log('No bin entries defined, skipping check.');
  process.exit(0);
}

const missing = bins.filter(name => {
  const filePath = path.resolve(__dirname, '..', require('../package.json').bin[name]);
  return !fs.existsSync(filePath);
});

if (missing.length > 0) {
  console.error(`Missing binary files: ${missing.join(', ')}`);
  process.exit(1);
}

console.log('All binary files present: ' + bins.join(', '));
process.exit(0);
