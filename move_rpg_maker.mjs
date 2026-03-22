import fs from 'fs';
import path from 'path';

const sourceDir = 'E:\\游戏 cosmos\\rpg-remaker\\compilers\\rpg-types\\src\\rpg_maker';
const targetDir = 'E:\\游戏 cosmos\\rpg-remaker\\compilers\\rpg-data\\src\\rpg_maker';

const files = [
    'actor.rs',
    'animation.rs',
    'armor.rs',
    'class_.rs',
    'enemy.rs',
    'event.rs',
    'item.rs',
    'map.rs',
    'mapinfo.rs',
    'mod.rs',
    'script.rs',
    'shared.rs',
    'skill.rs',
    'state.rs',
    'system.rs',
    'tileset.rs',
    'troop.rs',
    'weapon.rs'
];

if (!fs.existsSync(targetDir)) {
    fs.mkdirSync(targetDir, { recursive: true });
    console.log('Created target directory:', targetDir);
}

for (const file of files) {
    const sourcePath = path.join(sourceDir, file);
    const targetPath = path.join(targetDir, file);
    
    if (fs.existsSync(sourcePath)) {
        fs.copyFileSync(sourcePath, targetPath);
        console.log('Copied:', file);
    } else {
        console.log('Source not found:', file);
    }
}

console.log('\nDone! Files copied to:', targetDir);
console.log('\nNext steps:');
console.log('1. Update rpg-data/Cargo.toml - add serde, serde_json, base64 dependencies');
console.log('2. Update rpg-data/src/lib.rs - add mod rpg_maker');
console.log('3. Update rpg-types/src/lib.rs - remove mod rpg_maker');
console.log('4. Fix imports in other crates');
