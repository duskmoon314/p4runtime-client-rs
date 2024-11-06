.PHONY: clean start

mkdir: 
	mkdir -p build

clean:
	rm -rf build

build: mkdir p4src/main.p4
	cd build \
	&& cmake ${SDE}/p4studio/ \
		-DCMAKE_INSTALL_PREFIX=${SDE_INSTALL} \
		-DCMAKE_MODULE_PATH=${SDE}/cmake \
		-DP4_NAME=tna_p4rt_digest \
		-DP4_PATH=$(CURDIR)/p4src/main.p4 \
		-DP4FLAGS="--p4runtime-files=p4rt.bin,p4rt.txt,p4rt.json --p4runtime-force-std-externs" \
	&& make tna_p4rt_digest \
	&& make install

start:
	${SDE}/run_switchd.sh -p tna_p4rt_digest --conf-file ${SDE_INSTALL}/share/p4/targets/tofino/tofino_skip_p4.conf --skip-p4 --p4rt-server 0.0.0.0:9559