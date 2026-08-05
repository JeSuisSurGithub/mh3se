"use strict";

const idToType = [
    "NONE",
    "chest",
    "arms",
    "waist",
    "legs",
    "head",
    "talisman",
    "gs",
    "sns",
    "ha",
    "la",
    "frame",
    "barrel",
    "stock",
    "ls",
    "sa",
];

function wrapInt8(x) {
  x &= 0xFF;
  return x >= 0x80 ? x - 0x100 : x;
}

function wrapAdd8(a, b) {
  return wrapInt8(a + b);
}

function wrapSub8(a, b) {
  return wrapInt8(a - b);
}

function rgbToHex(r, g, b) {
    return "#" + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1).toUpperCase();
}

function hexToRgb(hex) {
    let bigint = parseInt(hex.slice(1), 16);
    let r = (bigint >> 16) & 255;
    let g = (bigint >> 8) & 255;
    let b = bigint & 255;
    return [r, g, b];
}

function getRowMeta(target) {
    const row = target.parentElement.parentElement;
    const [optName, index] = row.id.split("_slot");
    return { row, optName, index };
}

function dispatchInputEvent(rowId) {
    const event = new Event('input', { bubbles: true, cancelable: true });
    document.getElementById(rowId).dispatchEvent(event);
}

function onTypeChange(target) {
    const { row, optName, index } = getRowMeta(target);
    let typeId = Database.type.indexOf(target.value);

    let generator = generateBlankEquipSlot;
    let payload = { BlankEquipSlot: { buf: [0,0,0,0,0,0,0,0,0,0,0,0] } };

    if ((typeId >= 7 && typeId <= 10) || (typeId >= 14 && typeId <= 15)) {
        generator = generateMeleeWeapon;
        payload = { MeleeWeapon: { type_id: typeId, unused_lvl: 0, id: 0, unused_skill2_pt: 0, unused_skill1_pt: 0, deco1: 0, deco2: 0, deco3: 0 } };
    } else if (typeId >= 11 && typeId <= 13) {
        generator = generateRangedWeapon;
        payload = { RangedWeapon: { type_id: typeId, lvl: 0, id: 0, unused_skill2_pt: 0, unused_skill1_pt: 0, deco1: 0, deco2: 0, deco3: 0 } };
    } else if (typeId >= 1 && typeId <= 5) {
        generator = generateArmor;
        payload = { Armor: { type_id: typeId, lvl: 0, id: 0, unused_skill2_pt: 0, unused_skill1_pt: 0, deco1: 0, deco2: 0, deco3: 0 } };
    } else if (typeId === 6) {
        generator = generateCharm1Slot;
        payload = { Charm1Slot: { type_id: 6, slot_count: 1, id: 0, skill2_pt: 0, skill1_pt: 0, deco1: 0, skill1_id: 0, skill2_id: 0 } };
    }

    row.replaceWith(generator(payload, optName, index));
    dispatchInputEvent(row.id);
}

function onSlotCountChange(target) {
    const { row, optName, index } = getRowMeta(target);
    let slotCount = parseInt(target.value);

    const charmGenerators = [generateCharm0Slot, generateCharm1Slot, generateCharm2Slot, generateCharm3Slot];
    const generator = charmGenerators[slotCount];

    if (generator) {
        const key = `Charm${slotCount}Slot`;
        // Setup initial default templates dynamically based on chosen slot configurations
        const templates = [
            { type_id: 6, slot_count: 0, id: 0, skill2_pt: 0, skill1_pt: 0, skill1_id: 0, skill2_id: 0, unused_deco3: 0 },
            { type_id: 6, slot_count: 1, id: 0, skill2_pt: 0, skill1_pt: 0, deco1: 0, skill1_id: 0, skill2_id: 0 },
            { type_id: 6, slot_count: 2, id: 0, unused_skill2_pt: 0, skill1_pt: 0, deco1: 0, deco2: 0, skill1_id: 0 },
            { type_id: 6, slot_count: 3, id: 0, unused_skill2_pt: 0, unused_skill1_pt: 0, deco1: 0, deco2: 0, deco3: 0 }
        ];
        row.replaceWith(generator({ [key]: templates[slotCount] }, optName, index));
    } else {
        console.log("Something's wrong!");
    }
    dispatchInputEvent(row.id);
}

function generateBlankCell()
{
    const blankCell = document.createElement("td");
    const blankText = document.createElement("p");
    blankText.style.textAlign = "center";
    blankText.textContent = '';
    blankCell.appendChild(blankText);
    return blankCell;
}

function generateIntSelectCell(value, min, max)
{
    const intSelectCell = document.createElement("td");
    const intSelectInput = document.createElement("input");
    intSelectInput.type = "number";
    intSelectInput.min = min;
    intSelectInput.max = max;
    intSelectInput.value = value;
    intSelectCell.appendChild(intSelectInput);
    return intSelectCell;
}

function generateSelectCell(optList, optSelect)
{
    const suggestCell = document.createElement("td");
    const suggestInput = document.createElement("textarea");

    suggestInput.value = optList[optSelect];
    suggestInput.placeholder = "Search...";

    const suggestBox = document.createElement("div");
    suggestBox.className = "suggestions";

    suggestInput.oninput = function () {
        const query = this.value.toLowerCase();
        suggestBox.innerHTML = '';

        if (query.length > 0) {
            const inferedSuggest = optList.filter(item => item.toLowerCase().includes(query));
            inferedSuggest.forEach(suggestion => {
                const suggestionItem = document.createElement('div');
                suggestionItem.className = 'suggestion-item';
                suggestionItem.textContent = suggestion;
                suggestionItem.onclick = function () {
                    suggestInput.value = suggestion;
                    const event = new Event('input', {
                        bubbles: true,
                        cancelable: true
                    });
                    suggestInput.dispatchEvent(event);
                    suggestBox.innerHTML = '';
                };
                suggestBox.appendChild(suggestionItem);
            });
        }
    };

    suggestCell.appendChild(suggestInput);
    suggestCell.appendChild(suggestBox);

    return suggestCell;
}

function generateBlankEquipSlot(item, optName, index) {
    const row = document.createElement("tr");
    row.id = `${optName}_slot${index}`;
    row.oninput = function (event) {
        const typeSelectCell = row.childNodes[0].childNodes[0];
        if (event.target === typeSelectCell && Database.type.includes(typeSelectCell.value)) {
            onTypeChange(typeSelectCell);
        }
    };

    row.appendChild(generateSelectCell(Database.type, 0));

    for (let i = 0; i < 7; i++) {
        row.appendChild(generateBlankCell());
    }

    return row;
}

function generateCharm1Slot(item, optName, index) {
    let slotCount = item.Charm1Slot.slot_count;
    let skill1Pt = item.Charm1Slot.skill1_pt;
    let skill2Pt = item.Charm1Slot.skill2_pt;

    const row = document.createElement("tr");
    row.id = `${optName}_slot${index}`;
    row.oninput = function (event) {
        const typeSelectCell = row.childNodes[0].childNodes[0];
        if (event.target === typeSelectCell && Database.type.includes(typeSelectCell.value)) {
            onTypeChange(typeSelectCell);
        }

        const slotCountSelectCell = row.childNodes[1].childNodes[0];
        if (event.target === slotCountSelectCell) {
            onSlotCountChange(slotCountSelectCell);
        }

        SaveSlot.equip_box[index] = {
            Charm1Slot: {
                type_id: Database.type.indexOf(row.childNodes[0].childNodes[0].value),
                slot_count: 1,
                id: Database[idToType[[item.Charm1Slot.type_id]]].indexOf(row.childNodes[2].childNodes[0].value),
                skill2_pt: wrapAdd8(parseInt(row.childNodes[4].childNodes[0].value), 10),
                skill1_pt: wrapAdd8(parseInt(row.childNodes[3].childNodes[0].value), 10),
                deco1: Database.jewel.indexOf(row.childNodes[5].childNodes[0].value),
                skill1_id: Database.skill.indexOf(row.childNodes[6].childNodes[0].value),
                skill2_id: Database.skill.indexOf(row.childNodes[7].childNodes[0].value)
            }
        };
    };

    row.appendChild(generateSelectCell(Database.type, item.Charm1Slot.type_id));
    row.appendChild(generateIntSelectCell(slotCount, 0, 3));
    row.appendChild(generateSelectCell(Database[idToType[[item.Charm1Slot.type_id]]], item.Charm1Slot.id));
    row.appendChild(generateIntSelectCell(wrapSub8(skill1Pt, 10), -128, 127));
    row.appendChild(generateIntSelectCell(wrapSub8(skill2Pt, 10), -128, 127));
    row.appendChild(generateSelectCell(Database.jewel, item.Charm1Slot.deco1));
    row.appendChild(generateSelectCell(Database.skill, item.Charm1Slot.skill1_id));
    row.appendChild(generateSelectCell(Database.skill, item.Charm1Slot.skill2_id));

    return row;
}

function generateCharm0Slot(item, optName, index) {
    let slotCount = item.Charm0Slot.slot_count;
    let skill1Pt = item.Charm0Slot.skill1_pt;
    let skill2Pt = item.Charm0Slot.skill2_pt;

    const row = document.createElement("tr");
    row.id = `${optName}_slot${index}`;
    row.oninput = function (event) {
        const typeSelectCell = row.childNodes[0].childNodes[0];
        if (event.target === typeSelectCell && Database.type.includes(typeSelectCell.value)) {
            onTypeChange(typeSelectCell);
        }

        const slotCountSelectCell = row.childNodes[1].childNodes[0];
        if (event.target === slotCountSelectCell) {
            onSlotCountChange(slotCountSelectCell);
        }

        SaveSlot.equip_box[index] = {
            Charm0Slot: {
                type_id: Database.type.indexOf(row.childNodes[0].childNodes[0].value),
                slot_count: 0,
                id: Database[idToType[[item.Charm0Slot.type_id]]].indexOf(row.childNodes[2].childNodes[0].value),
                skill2_pt: wrapAdd8(parseInt(row.childNodes[4].childNodes[0].value), 10),
                skill1_pt: wrapAdd8(parseInt(row.childNodes[3].childNodes[0].value), 10),
                skill1_id: Database.skill.indexOf(row.childNodes[5].childNodes[0].value),
                skill2_id: Database.skill.indexOf(row.childNodes[6].childNodes[0].value),
                unused_deco3: 0
            }
        };
    };

    row.appendChild(generateSelectCell(Database.type, item.Charm0Slot.type_id));
    row.appendChild(generateIntSelectCell(slotCount, 0, 3));
    row.appendChild(generateSelectCell(Database[idToType[[item.Charm0Slot.type_id]]], item.Charm0Slot.id));
    row.appendChild(generateIntSelectCell(wrapSub8(skill1Pt, 10), -128, 127));
    row.appendChild(generateIntSelectCell(wrapSub8(skill2Pt, 10), -128, 127));
    row.appendChild(generateSelectCell(Database.skill, item.Charm0Slot.skill1_id));
    row.appendChild(generateSelectCell(Database.skill, item.Charm0Slot.skill2_id));
    row.appendChild(generateBlankCell());

    return row;
}

function generateCharm2Slot(item, optName, index) {
    let slotCount = item.Charm2Slot.slot_count;
    let skill1Pt = item.Charm2Slot.skill1_pt;

    const row = document.createElement("tr");
    row.id = `${optName}_slot${index}`;
    row.oninput = function (event) {
        const typeSelectCell = row.childNodes[0].childNodes[0];
        if (event.target === typeSelectCell && Database.type.includes(typeSelectCell.value)) {
            onTypeChange(typeSelectCell);
        }

        const slotCountSelectCell = row.childNodes[1].childNodes[0];
        if (event.target === slotCountSelectCell) {
            onSlotCountChange(slotCountSelectCell);
        }

        SaveSlot.equip_box[index] = {
            Charm2Slot: {
                type_id: Database.type.indexOf(row.childNodes[0].childNodes[0].value),
                slot_count: 2,
                id: Database[idToType[[item.Charm2Slot.type_id]]].indexOf(row.childNodes[2].childNodes[0].value),
                unused_skill2_pt: 0,
                skill1_pt: wrapAdd8(parseInt(row.childNodes[3].childNodes[0].value), 10),
                deco1: Database.jewel.indexOf(row.childNodes[5].childNodes[0].value),
                deco2: Database.jewel.indexOf(row.childNodes[6].childNodes[0].value),
                skill1_id: Database.skill.indexOf(row.childNodes[7].childNodes[0].value)
            }
        };
    };

    row.appendChild(generateSelectCell(Database.type, item.Charm2Slot.type_id));
    row.appendChild(generateIntSelectCell(slotCount, 0, 3));
    row.appendChild(generateSelectCell(Database[idToType[[item.Charm2Slot.type_id]]], item.Charm2Slot.id));
    row.appendChild(generateIntSelectCell(wrapSub8(skill1Pt, 10), -128, 127));
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database.jewel, item.Charm2Slot.deco1));
    row.appendChild(generateSelectCell(Database.jewel, item.Charm2Slot.deco2));
    row.appendChild(generateSelectCell(Database.skill, item.Charm2Slot.skill1_id));

    return row;
}

function generateCharm3Slot(item, optName, index) {
    let slotCount = item.Charm3Slot.slot_count;

    const row = document.createElement("tr");
    row.id = `${optName}_slot${index}`;
    row.oninput = function (event) {
        const typeSelectCell = row.childNodes[0].childNodes[0];
        if (event.target === typeSelectCell && Database.type.includes(typeSelectCell.value)) {
            onTypeChange(typeSelectCell);
        }

        const slotCountSelectCell = row.childNodes[1].childNodes[0];
        if (event.target === slotCountSelectCell) {
            onSlotCountChange(slotCountSelectCell);
        }

        SaveSlot.equip_box[index] = {
            Charm3Slot: {
                type_id: Database.type.indexOf(row.childNodes[0].childNodes[0].value),
                slot_count: 3,
                id: Database[idToType[[item.Charm3Slot.type_id]]].indexOf(row.childNodes[2].childNodes[0].value),
                unused_skill2_pt: 0,
                unused_skill1_pt: 0,
                deco1: Database.jewel.indexOf(row.childNodes[5].childNodes[0].value),
                deco2: Database.jewel.indexOf(row.childNodes[6].childNodes[0].value),
                deco3: Database.jewel.indexOf(row.childNodes[7].childNodes[0].value)
            }
        };
    };

    row.appendChild(generateSelectCell(Database.type, item.Charm3Slot.type_id));
    row.appendChild(generateIntSelectCell(slotCount, 0, 3));
    row.appendChild(generateSelectCell(Database[idToType[[item.Charm3Slot.type_id]]], item.Charm3Slot.id));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database.jewel, item.Charm3Slot.deco1));
    row.appendChild(generateSelectCell(Database.jewel, item.Charm3Slot.deco2));
    row.appendChild(generateSelectCell(Database.jewel, item.Charm3Slot.deco3));

    return row;
}

function generateArmor(item, optName, index) {
    let equipLevel = item.Armor.lvl;

    const row = document.createElement("tr");
    row.id = `${optName}_slot${index}`;
    row.oninput = function (event) {
        const typeSelectCell = row.childNodes[0].childNodes[0];
        if (event.target === typeSelectCell && Database.type.includes(typeSelectCell.value)) {
            onTypeChange(typeSelectCell);
        }

        SaveSlot.equip_box[index] = {
            Armor: {
                type_id: Database.type.indexOf(row.childNodes[0].childNodes[0].value),
                lvl: parseInt(row.childNodes[1].childNodes[0].value),
                id: Database[idToType[[item.Armor.type_id]]].indexOf(row.childNodes[2].childNodes[0].value),
                unused_skill2_pt: 0,
                unused_skill1_pt: 0,
                deco1: Database.jewel.indexOf(row.childNodes[5].childNodes[0].value),
                deco2: Database.jewel.indexOf(row.childNodes[6].childNodes[0].value),
                deco3: Database.jewel.indexOf(row.childNodes[7].childNodes[0].value)
            }
        };
    };

    row.appendChild(generateSelectCell(Database.type, item.Armor.type_id));
    row.appendChild(generateIntSelectCell(equipLevel + 1, 0, 32));
    row.appendChild(generateSelectCell(Database[idToType[[item.Armor.type_id]]], item.Armor.id));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database.jewel, item.Armor.deco1));
    row.appendChild(generateSelectCell(Database.jewel, item.Armor.deco2));
    row.appendChild(generateSelectCell(Database.jewel, item.Armor.deco3));

    return row;
}

function generateMeleeWeapon(item, optName, index) {
    const row = document.createElement("tr");
    row.id = `${optName}_slot${index}`;
    row.oninput = function (event) {
        const typeSelectCell = row.childNodes[0].childNodes[0];
        if (event.target === typeSelectCell && Database.type.includes(typeSelectCell.value)) {
            onTypeChange(typeSelectCell);
        }

        SaveSlot.equip_box[index] = {
            MeleeWeapon: {
                type_id: Database.type.indexOf(row.childNodes[0].childNodes[0].value),
                unused_lvl: 0,
                id: Database[idToType[[item.MeleeWeapon.type_id]]].indexOf(row.childNodes[2].childNodes[0].value),
                unused_skill2_pt: 0,
                unused_skill1_pt: 0,
                deco1: Database.jewel.indexOf(row.childNodes[5].childNodes[0].value),
                deco2: Database.jewel.indexOf(row.childNodes[6].childNodes[0].value),
                deco3: Database.jewel.indexOf(row.childNodes[7].childNodes[0].value)
            }
        };
    };

    row.appendChild(generateSelectCell(Database.type, item.MeleeWeapon.type_id));
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database[idToType[[item.MeleeWeapon.type_id]]], item.MeleeWeapon.id));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database.jewel, item.MeleeWeapon.deco1));
    row.appendChild(generateSelectCell(Database.jewel, item.MeleeWeapon.deco2));
    row.appendChild(generateSelectCell(Database.jewel, item.MeleeWeapon.deco3));

    return row;
}

function generateRangedWeapon(item, optName, index) {
    let equipLevel = item.RangedWeapon.lvl;

    const row = document.createElement("tr");
    row.id = `${optName}_slot${index}`;
    row.oninput = function (event) {
        const typeSelectCell = row.childNodes[0].childNodes[0];
        if (event.target === typeSelectCell && Database.type.includes(typeSelectCell.value)) {
            onTypeChange(typeSelectCell);
        }

        SaveSlot.equip_box[index] = {
            RangedWeapon: {
                type_id: Database.type.indexOf(row.childNodes[0].childNodes[0].value),
                lvl: parseInt(row.childNodes[1].childNodes[0].value),
                id: Database[idToType[[item.RangedWeapon.type_id]]].indexOf(row.childNodes[2].childNodes[0].value),
                unused_skill2_pt: 0,
                unused_skill1_pt: 0,
                deco1: Database.jewel.indexOf(row.childNodes[5].childNodes[0].value),
                deco2: Database.jewel.indexOf(row.childNodes[6].childNodes[0].value),
                deco3: Database.jewel.indexOf(row.childNodes[7].childNodes[0].value)
            }
        };
    };

    row.appendChild(generateSelectCell(Database.type, item.RangedWeapon.type_id));
    row.appendChild(generateIntSelectCell(equipLevel + 1, 0, 32));
    row.appendChild(generateSelectCell(Database[idToType[[item.RangedWeapon.type_id]]], item.RangedWeapon.id));
    row.appendChild(generateBlankCell());
    row.appendChild(generateBlankCell());
    row.appendChild(generateSelectCell(Database.jewel, item.RangedWeapon.deco1));
    row.appendChild(generateSelectCell(Database.jewel, item.RangedWeapon.deco2));
    row.appendChild(generateSelectCell(Database.jewel, item.RangedWeapon.deco3));

    return row;
}

function generateItemList(optName, optData) {
    const tbody = document.querySelector(`#${optName} tbody`);
    optData.forEach((data, index) => {
        const row = document.createElement("tr");
        row.id = `${optName}_slot${index}`;
        row.oninput = function(event) {
            if (event.target.tagName.toLowerCase() === "textarea" ) {
                SaveSlot[optName][index].id = Database.item.indexOf(event.target.value);
            }
            else if (event.target.tagName.toLowerCase() === "input")
            {
                SaveSlot[optName][index].qty = parseInt(event.target.value);
            }
        };

        row.appendChild(generateSelectCell(Database.item, data.id));
        row.appendChild(generateIntSelectCell(data.qty, 0, 99));

        tbody.appendChild(row);
    });
}

function generateEquipBox(optName, optData) {
    const tbody = document.querySelector(`#${optName} tbody`);

    optData.forEach((item, index) => {
        if (item.hasOwnProperty('MeleeWeapon')) {
            tbody.appendChild(generateMeleeWeapon(item, optName, index));

        } else if (item.hasOwnProperty('RangedWeapon')) {
            tbody.appendChild(generateRangedWeapon(item, optName, index));

        } else if (item.hasOwnProperty('Armor')) {
            tbody.appendChild(generateArmor(item, optName, index));

        } else if (item.hasOwnProperty('Charm0Slot')) {
            tbody.appendChild(generateCharm0Slot(item, optName, index));

        } else if (item.hasOwnProperty('Charm1Slot')) {
            tbody.appendChild(generateCharm1Slot(item, optName, index));

        } else if (item.hasOwnProperty('Charm2Slot')) {
            tbody.appendChild(generateCharm2Slot(item, optName, index));

        } else if (item.hasOwnProperty('Charm3Slot')) {
            tbody.appendChild(generateCharm3Slot(item, optName, index));

        } else if (item.hasOwnProperty('BlankEquipSlot')) {
            tbody.appendChild(generateBlankEquipSlot(item, optName, index));

        } else {
            console.log("Something's Wrong!");
        }
    });
}

function swapPage(target) {
    let pageId = ["melee_pouch", "ranged_pouch", "item_box" ,"equip_box"][parseInt(target.value)];

    if (lastChoice != "" && lastChoice != pageId) {
        document.getElementById(lastChoice).style.display = "none";
    }

    document.getElementById(pageId).style.display = "block";

    lastChoice = pageId;
}

function initEditing() {
    // Read

    const gender = document.getElementById("gender")
    gender.value = SaveSlot.gender;
    gender.onchange = function () {
        SaveSlot.gender = parseInt(gender.value);
    };

    const name = document.getElementById("name")
    name.value = String.fromCharCode.apply(null, SaveSlot.name.filter((code) => !(code === 0)));
    name.onchange = function () {
        const charCodes = Array.from(name.value).map(char => char.charCodeAt(0));
        SaveSlot.name = charCodes.concat(new Array(8 - charCodes.length).fill(0)).slice(0, 8);
    };

    const zenny = document.getElementById("zenny")
    zenny.value = SaveSlot.zenny;
    zenny.onchange = function () {
        SaveSlot.zenny = parseInt(zenny.value);
    };

    const playtime = document.getElementById("playtime")
    playtime.value = SaveSlot.playtime;
    playtime.onchange = function () {
        SaveSlot.playtime = parseInt(playtime.value);
    };

    const hrp = document.getElementById("hrp")
    hrp.value = SaveSlot.hrp;
    hrp.onchange = function () {
        SaveSlot.hrp = parseInt(hrp.value);
    };

    const hr = document.getElementById("hr")
    hr.value = SaveSlot.hr;
    hr.onchange = function () {
        SaveSlot.hr = parseInt(hr.value);
    };

    const face_type = document.getElementById("face_type")
    face_type.value    = SaveSlot.face_type + 1;
    face_type.onchange = function () {
        SaveSlot.face_type = parseInt(face_type.value - 1);
    };

    const hair_type = document.getElementById("hair_type")
    hair_type.value    = SaveSlot.hair_type + 1;
    hair_type.onchange = function () {
        SaveSlot.hair_type = parseInt(hair_type.value - 1);
    };

    const hair_color = document.getElementById("hair_color")
    hair_color.value   = rgbToHex(SaveSlot.hair_color[0], SaveSlot.hair_color[1], SaveSlot.hair_color[2]);
    hair_color.onchange = function () {
        SaveSlot.hair_color = hexToRgb(hair_color.value);
    };

    const cloth_type = document.getElementById("cloth_type")
    cloth_type.value   = SaveSlot.cloth_type + 1;
    cloth_type.onchange = function () {
        SaveSlot.cloth_type = parseInt(cloth_type.value - 1);
    };

    const voice_type = document.getElementById("voice_type")
    voice_type.value   = SaveSlot.voice_type + 1;
    voice_type.onchange = function () {
        SaveSlot.voice_type = parseInt(voice_type.value - 1);
    };

    const cloth_color = document.getElementById("cloth_color")
    cloth_color.value  = rgbToHex(SaveSlot.cloth_color[0], SaveSlot.cloth_color[1], SaveSlot.cloth_color[2]);
    cloth_color.onchange = function () {
        SaveSlot.cloth_color = hexToRgb(cloth_color.value);
    };

    const eye_color = document.getElementById("eye_color")
    eye_color.value    = SaveSlot.eye_color + 1;
    eye_color.onchange = function () {
        SaveSlot.eye_color = parseInt(eye_color.value - 1);
    };

    const feature_type = document.getElementById("feature_type")
    feature_type.value = SaveSlot.feature_type;
    feature_type.onchange = function () {
        SaveSlot.feature_type = parseInt(feature_type.value);
    };

    const skin_tone = document.getElementById("skin_tone")
    skin_tone.value    = SaveSlot.skin_tone;
    skin_tone.onchange = function () {
        SaveSlot.skin_tone = parseInt(skin_tone.value);
    };

    generateItemList("melee_pouch", SaveSlot.melee_pouch);
    generateItemList("ranged_pouch", SaveSlot.ranged_pouch);
    generateItemList("item_box", SaveSlot.item_box);
    generateEquipBox("equip_box", SaveSlot.equip_box);

    swapPage(document.getElementById("page"));
}

async function loadJson(url) {
    try {
        const response = await fetch(`${window.location.origin}/${url}`);

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        const jsonObject = await response.json();

        return jsonObject;
    } catch (error) {
        console.error('Error fetching JSON data:', error);
        return null;
    }
}

function axumPost(suffix, data) {
    return fetch(window.location.origin + suffix, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    })
    .then(response => response.json())
    .then(json_data => { return json_data; })
    .catch(error => { console.error('Error:', error); });
}

async function save2json() {
    let binfile_tag = document.getElementById("load_binfile");
    let slot_tag = document.getElementById("load_slot");

    let info = "";
    if (binfile_tag.files.length != 1) {
        info += "You didn't upload any save file!\n";
    }
    if (slot_tag.value < 1 || slot_tag.value > 3) {
        info += "Slot number invalid!\n";
    }
    if (info != "") {
        window.alert(info);
        return;
    }

    const savebinfname = binfile_tag.files[0];
    const lecteur = new FileReader();
    lecteur.onloadend = async function (event) {
        const binfile = event.target.result.split(',')[1];
        const slot = slot_tag.value - 1;
        const res = await axumPost("/save2json", { binfile, slot });
        if (res === undefined) {
            window.alert("Error: Please check the command line logs!");
            return;
        }
        if (res["status"] === "ERR") {
            window.alert(res["payload"]);
            return;
        }
        SaveSlot = JSON.parse(res["payload"]);
        initEditing();

        window.alert("Loaded save file!")
    };
    lecteur.readAsDataURL(savebinfname);
}

async function json2save() {
    let binfile_tag = document.getElementById("build_binfile");
    let slot_tag = document.getElementById("build_slot");

    let info = "";
    if (binfile_tag.files.length != 1) {
        info += "You didn't upload any save file!\n";
    }
    if (slot_tag.value < 1 || slot_tag.value > 3) {
        info += "Slot number invalid!\n";
    }
    if (info != "") {
        window.alert(info);
        return;
    }

    const savebinfname = binfile_tag.files[0];
    const lecteur = new FileReader();
    lecteur.onloadend = async function (event) {
        const binfile = event.target.result.split(',')[1];
        const jsonfile = JSON.stringify(SaveSlot);
        const slot = slot_tag.value - 1;
        const res = await axumPost("/json2save", { binfile, jsonfile, slot });
        if (res === undefined) {
            window.alert("Error: Please check the command line logs!");
            return;
        }
        if (res["status"] === "ERR") {
            window.alert(res["payload"]);
            return;
        }

        const byteCharacters = atob(res["payload"]);
        const byteNumbers = new Array(byteCharacters.length);

        for (let i = 0; i < byteCharacters.length; i++) {
            byteNumbers[i] = byteCharacters.charCodeAt(i);
        }

        const byteArray = new Uint8Array(byteNumbers);
        const blob = new Blob([byteArray], { type: 'application/octet-stream' });

        const link = document.createElement('a');
        link.href = URL.createObjectURL(blob);
        link.download = "data00";

        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    };
    lecteur.readAsDataURL(savebinfname);
}

let lastChoice = "";
let Database = {};
let SaveSlot = {};

(async () => {
    Database = await loadJson('web/database.json');
})();