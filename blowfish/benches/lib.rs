#![no_std]
#![feature(test)]

block_cipher::bench!(blowfish::Blowfish, 16);
