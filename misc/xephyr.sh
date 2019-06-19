Xephyr -br -ac -noreset -screen 1280x800 :1 &
sleep 1
DISPLAY=:1 $1
