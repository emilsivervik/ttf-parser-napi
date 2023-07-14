const fs = require('fs')
// const { TTFParser } = require('./pkg/ttf_parser_wasm')
// const opentype = require('opentype.js')
// const fontPkg = require('font');
// const fontName = require('fontname')
const { TTFParser } = require('./index')

const file = fs.readFileSync('./ABBvoice-Medium.ttf');
// const file = fs.readFileSync('./NotoSansCJK.ttf');
// const file = fs.readFileSync('./arial-unicode-ms.ttf');
// const file = fs.readFileSync('./Aeonik-Bold.otf');
// const file = fs.readFileSync('./Aeonik-Regular.otf');

console.log(file)
try {
  console.time('TTFParser Font')
  const font = new TTFParser(file);
  console.timeEnd('TTFParser Font')
  console.log(font.ascender)
  console.log(font.height)
  console.log(font)
  console.log(JSON.stringify(font))
  // console.log(font.tables.name.names.forEach(v => console.log(v)))
  // console.log(font.tables.os2)
  // console.log(font.tables.head)
  // console.log(font.tables.head.global_bbox)
} catch (err) {
  console.log(err, 'first failed')
}
