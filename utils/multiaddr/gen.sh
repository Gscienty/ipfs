#!/bin/sh

curl 'https://raw.githubusercontent.com/multiformats/multiaddr/master/protocols.csv' |\
    awk '
BEGIN {
    firstline=true;
    nth=0;
}
{
        name[nth] = substr($1, 0, length($1)-1);
        value[nth] = substr($3, 0, length($3)-1);

        nth = nth + 1;
}
END {
    print "pub fn to_type(val: u64) -> String {"
    print "    match val {";
    for (i = 1; i < nth; i++) {
        print "        " name[i] " => \"" value[i] "\",";
    }
    print "        _ => unreachable!(),";
    print "    }"
    print "    .to_string()"
    print "}"

    print ""

    print "pub fn to_code(val: &str) -> u64 {"
    print "    match val {";
    for (i = 1; i < nth; i++) {
        print "        \"" value[i] "\" => " name[i] ",";
    }
    print "        _ => unreachable!(),";
    print "    }"
    print "}"
}' > ./src/protocols.rs
