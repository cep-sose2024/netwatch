
.PHONY: build
build:
	cargo build --target aarch64-linux-android --release
	cp /home/mark/greetings/rust/target/aarch64-linux-android/release/librust.so /home/mark/greetings/android/Greetings/app/src/main/jniLibs/arm64-v8a/librust.so
	cargo build --target armv7-linux-androideabi --release
	cp /home/mark/greetings/rust/target/armv7-linux-androideabi/release/librust.so /home/mark/greetings/android/Greetings/app/src/main/jniLibs/armeabi-v7a/librust.so
	cargo build --target i686-linux-android --release
	cp /home/mark/greetings/rust/target/i686-linux-android/release/librust.so /home/mark/greetings/android/Greetings/app/src/main/jniLibs/x86/librust.so
