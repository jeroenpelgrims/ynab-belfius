import { exit } from "process";
import { readFileSync, writeFileSync } from "fs";
import { dropWhile } from "ramda";

function parseCsv(data: string) {
    // WHY dropWhile: The first 13 or so lines in the Belfius CSV are a list of filters used for the export (I assume)
    // We're skipping these so we are left with the actual transaction lines.
    const lines = dropWhile(line => !line.startsWith("BE"), data.split("\n"));
    const objects = lines.map(line => line.split(";"));
    const wantedFields = objects.map(o => [
        o[1], // Boekingsdatum
        o[5], // Naam tegenpartij bevat
        o[14].trim(), // Mededeling
        o[10], // Bedrag
    ]);

    const output = wantedFields.map(o => o.join(";"));
    const headers = "Date;Payee;Memo;Amount";

    return [headers, ...output].join("\n");
}

const arg = process.argv;
const path = arg[arg.length - 1];

if (!path || !path.match(/\.csv$/)) {
    console.log("First argument needs to be the Belfius exported csv");
    exit(1);
} else {
    const data = readFileSync(path).toString();
    const converted = parseCsv(data);
    const outputPath = path.replace(/\.csv/, ".ynab.csv");

    writeFileSync(outputPath, converted);
    console.log(`File converted and saved at ${outputPath}`);
}