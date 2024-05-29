#!/bin/sh

curl http://192.168.0.26:8080/test;

curl http://192.168.0.26:8080/createemptyplaylist/testplaylist;

curl http://192.168.0.26:8080/createrandomplaylist/testplaylist2/25;

curl http://192.168.0.26:8080/allplaylists;

curl http://192.168.0.26:8080/randomcoverart;