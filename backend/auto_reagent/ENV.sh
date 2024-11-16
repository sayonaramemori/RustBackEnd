#source this file
IP=`ifconfig | awk '/eth0/{r=NR+1}NR==r{print $2}'`
export IP=$IP
echo $IP

