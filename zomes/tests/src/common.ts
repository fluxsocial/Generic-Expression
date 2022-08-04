import path from "path";
import {Dna} from "@holochain/tryorama";

const dnas: Dna[] = [{ source: {path: path.join("../../workdir/generic-expression.dna")} }];

const sleep = (ms: number) =>
    new Promise((resolve) => setTimeout(() => resolve(null), ms));

export { dnas, sleep };