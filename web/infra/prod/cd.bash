#!/bin/bash
# Who needs continuous deployment when you have this!

ssh ubuntu@paltamaster.ddns.net "cd docker-apps/nes-emu/web/infra/prod && sudo docker-compose pull && sudo docker-compose down && sudo docker-compose up -d"
