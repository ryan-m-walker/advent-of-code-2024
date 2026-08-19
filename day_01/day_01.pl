#!/usr/bin/env perl

use strict;
use warnings;

my $file = shift // 'input.txt';

open my $fh, '<', $file or die "Failed to read input file $file: $!";

my (@left, @right);

while (my $line = <$fh>) {
    next unless $line =~ /\S/;
    my ($l, $r) = split ' ', $line;
    push @left,  $l;
    push @right, $r;
}

close $fh;

print "part 1: ", part_01(\@left, \@right), "\n";
print "part 2: ", part_02(\@left, \@right), "\n";

sub part_01 {
    my ($left, $right) = @_;

    my @sorted_left  = sort { $a <=> $b } @$left;
    my @sorted_right = sort { $a <=> $b } @$right;

    my $sum = 0;

    for my $i (0 .. $#sorted_left) {
        $sum += abs($sorted_right[$i] - $sorted_left[$i]);
    }

    return $sum;
}

sub part_02 {
    my ($left, $right) = @_;

    my %occurrences;
    $occurrences{$_}++ for @$right;

    my $sum = 0;

    for my $l (@$left) {
        $sum += $l * ($occurrences{$l} // 0);
    }

    return $sum;
}
