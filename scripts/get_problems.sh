#!/bin/bash

PROB="spaceship"

mkdir -p ../courses/$PROB/raw

for id in {3..25}
do
  echo "get $PROB$id"
  echo "get $PROB$id" | ruby encode-string.rb | ruby communicate.rb > "../courses/$PROB/raw/$PROB$id"
  cat ../courses/$PROB/raw/$PROB$id | ruby decode-string.rb > "../courses/$PROB/$PROB$id"
  sleep 4
done