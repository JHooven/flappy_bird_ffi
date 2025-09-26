#!/usr/bin/perl
use strict;
use warnings;

package main;
my $name = "";
my $version = "";
my $lastLine = "";
while (my $line = <>) {
    chomp $line;

    if ($line =~ /^Compiling\s(\w*)\sv(.*)$/) {
            $name = $1;
            $version = $2;
            print "Processing $name version $version\n";
    }

    if ($line =~ /^error/ and $version ne "") {
        print "$name = { version = \"$version\", optional = true }\n";
        $version = "";

    } 
}


print "Finshed\n";