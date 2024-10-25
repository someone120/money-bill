import en_US from "./assets/i18n/en.json";
import zh_CN from "./assets/i18n/zh_CN.json";

export class i18n {
  private static instance: i18n;
  private localeId: string = "en_US";
  private constructor(localeId:string) {
    this.localeId = localeId;
    // ..
  }
  getString(key: string) {
    let locales = { en_US: en_US, zh_CN: zh_CN };
    console.log(locales[this.localeId] ?? locales["en_US"]);
    console.log((locales[this.localeId] ?? locales["en_US"])[key]);

    return (locales[this.localeId] ?? locales["en_US"])[key];
  }

  changeLocale(localeId:string) {
    this.localeId = localeId || "en_US";
  }

  public static getInstace(localeId:string) {
    if (!i18n.instance) {
      i18n.instance = new i18n(localeId);
    }

    return i18n.instance;
  }
}
