let _ = require("lodash");
let types = require("./types");
let fs = require("fs");

// gen_datatypes.js generates a rust module with the structs and enum from a .bsd file.

let argv = require("yargs")
    .usage("Usage: $0 --bsd [path] --module [name]")
    .demandOption(['bsd', 'module'])
    .describe('bsd', "The OPC UA Bsd file to parse")
    .describe('module', "Path to the module folder.")
    .argv;

let bsd_file = argv.bsd;
let rs_module = argv.module;

if (!fs.existsSync(rs_module)) {
    fs.mkdirSync(rs_module, {recursive: true});
}

let codegen_info = {
    autogen_comment: `This file was autogenerated from ${bsd_file} by tools/schema/gen_datatypes.js`,
    types_crate: "opcua::types",
    custom_bsd: true,
};

types.from_xml(bsd_file, rs_module, codegen_info);
