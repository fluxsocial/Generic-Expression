import type { Address, Agent, Language, HolochainLanguageDelegate, LanguageContext, Interaction} from "@perspect3vism/ad4m";
import GenericExpressionAdapter from "./adapter";
import GenericExpressionAuthorAdapter from "./authorAdapter";
import Icon from "./build/Icon.js";
import ConstructorIcon from "./build/ConstructorIcon.js";
import { JuntoSettingsUI } from "./SettingsUI";
import { GenericExpressionUI } from "./genericExpressionUI";
import { DNA, DNA_NICK } from "./dna";

function iconFor(expression: Address): string {
  return Icon;
}

function constructorIcon(): string {
  return ConstructorIcon;
}

function interactions(expression: Address): Interaction[] {
  return [];
}

export const name = "generic-expression";

export default async function create(context: LanguageContext): Promise<Language> {
  const Holochain = context.Holochain as HolochainLanguageDelegate;
  await Holochain.registerDNAs([{ file: DNA, nick: DNA_NICK }]);

  const expressionAdapter = new GenericExpressionAdapter(context);
  const authorAdaptor = new GenericExpressionAuthorAdapter(context);
  const settingsUI = new JuntoSettingsUI();
  const expressionUI = new GenericExpressionUI();

  return {
    name,
    expressionAdapter,
    authorAdaptor,
    iconFor,
    constructorIcon,
    interactions,
    settingsUI,
    expressionUI,
  } as Language;
}
