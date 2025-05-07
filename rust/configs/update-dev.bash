main() {
    curl -L https://api.myrts.qcynaut.tech/install/myrts-client -o /tmp/myrts-client
    chmod +x /tmp/myrts-client
    mv /tmp/myrts-client /var/lib/myrts/
    systemctl restart myrts-client && systemctl enable myrts-client
}

main