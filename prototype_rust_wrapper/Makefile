
.PHONY: build
build:
	cargo build --target aarch64-linux-android --release
	mkdir -p ../prototype/app/src/main/jniLibs/arm64-v8a
	cp target/aarch64-linux-android/release/libprototype_rust_wrapper.so ../prototype/app/src/main/jniLibs/arm64-v8a/libprototype_rust_wrapper.so

	cargo build --target armv7-linux-androideabi --release
	mkdir -p ../prototype/app/src/main/jniLibs/armeabi-v7a
	cp target/armv7-linux-androideabi/release/libprototype_rust_wrapper.so ../prototype/app/src/main/jniLibs/armeabi-v7a/libprototype_rust_wrapper.so

	cargo build --target i686-linux-android --release
	mkdir -p ../prototype/app/src/main/jniLibs/x86
	cp target/i686-linux-android/release/libprototype_rust_wrapper.so ../prototype/app/src/main/jniLibs/x86/libprototype_rust_wrapper.so
