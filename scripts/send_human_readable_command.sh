#!/bin/bash
echo $1 | ruby encode-string.rb | ruby communicate.rb | ruby decode-string.rb
