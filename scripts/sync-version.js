#!/usr/bin/env node

import { readFileSync, writeFileSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// 读取版本信息
const versionInfo = JSON.parse(readFileSync(resolve(__dirname, '../version.json'), 'utf-8'));

// 更新 package.json
const packageJsonPath = resolve(__dirname, '../package.json');
const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf-8'));
packageJson.version = versionInfo.version;
packageJson.name = versionInfo.name.toLowerCase().replace(/\s+/g, '-');
packageJson.description = versionInfo.description;
writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2) + '\n');

// 更新 tauri.conf.json
const tauriConfPath = resolve(__dirname, '../src-tauri/tauri.conf.json');
const tauriConf = JSON.parse(readFileSync(tauriConfPath, 'utf-8'));
tauriConf.version = versionInfo.version;
tauriConf.productName = versionInfo.name;
tauriConf.identifier = `com.${versionInfo.author.toLowerCase()}.${versionInfo.name.toLowerCase().replace(/\s+/g, '')}`;
writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');

// 更新 Cargo.toml
const cargoTomlPath = resolve(__dirname, '../src-tauri/Cargo.toml');
let cargoToml = readFileSync(cargoTomlPath, 'utf-8');
cargoToml = cargoToml.replace(/^version = ".*"$/m, `version = "${versionInfo.version}"`);
cargoToml = cargoToml.replace(/^name = ".*"$/m, `name = "${versionInfo.name.toLowerCase().replace(/\s+/g, '-')}"`);
cargoToml = cargoToml.replace(/^description = ".*"$/m, `description = "${versionInfo.description}"`);
cargoToml = cargoToml.replace(/^authors = \[".*"\]$/m, `authors = ["${versionInfo.author}"]`);
writeFileSync(cargoTomlPath, cargoToml);

console.log('✅ 版本信息已同步到所有配置文件');
console.log(`📦 版本: ${versionInfo.version}`);
console.log(`📝 名称: ${versionInfo.name}`);
console.log(`👤 作者: ${versionInfo.author}`);
console.log(`📄 许可证: ${versionInfo.license}`);
