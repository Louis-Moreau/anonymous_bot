#!/bin/bash
export TERM='vt100'
screen -S bot -d -m anonymous_bot
gotty -p 8080 screen -x bot