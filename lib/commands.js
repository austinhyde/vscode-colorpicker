const vscode = require('vscode');
const path = require('path');
const {spawn} = require('child_process');

// matches things like
// #F12
// #F128
// #FF112288
// rgba(255, 17, 34, .53)
// hsl(90deg, .2, .5)
const colorRegexp = /((rgb|hsl)a? *\( *(\d+(\.\d+)? *(%|deg|rad|grad|turn)?[,\/ ] *?){3}\d+(\.\d+)? *%?\))|(#[0-9A-F]{3,8})/i;

exports.showPicker = function() {
  try {
    const textEditor = vscode.window.activeTextEditor;
    if (!textEditor) {
      console.warn('No active text editor.');
      return;
    }

    const pickerPath = path.resolve(path.dirname(module.filename) + '/../dist/picker');
    const wordRange = textEditor.document.getWordRangeAtPosition(textEditor.selection.anchor, colorRegexp);
    const color = textEditor.document.getText(wordRange);
    const config = vscode.workspace.getConfiguration();
    const font = config.get('editor.fontFamily');
    const fontSize = config.get('editor.fontSize');

    console.log({pickerPath, color, font, fontSize});
    let res = spawn(pickerPath, [color, '--font', font, '--font-size', fontSize]);
    console.log(res.stdout.toString());
  } catch (e) {
    vscode.window.showErrorMessage(e+'');
    console.error(e);
  }
};