import enUS from "./en-US/index.js"; // "./en-US";
import zhCN from "./zh-CN/index.js"; // "./zh-CN";

let _Language = {};

export function loadLang(lang) {
  switch (lang) {
  case "zh-CN":
    _Language = zhCN;
    break;
  case "en-US":
    _Language = enUS;
    break;
  default:
    _Language = enUS;
  }
  return _Language;
}

export function getSet(key) {
  return _Language[key] || {};
}
