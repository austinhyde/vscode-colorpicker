const { commands } = require('vscode');
const packageJSON = require('./package');
const extCommands = require('./lib/commands');

exports.activate = function (context) {
	packageJSON.contributes.commands.forEach(({command}) => {
		let disposable = commands.registerCommand(command, extCommands[command.split('.').slice(-1)]);
		context.subscriptions.push(disposable);
	});
}

exports.deactivate = function () {}
