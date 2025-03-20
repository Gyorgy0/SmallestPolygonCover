export ANDROID_HOME=$HOME/Android/Sdk
export PATH=$PATH:$ANDROID_HOME/tools
export ANDROID_NDK_ROOT=$HOME/Android/Sdk/ndk
ANDROID_HOME=$ANDROID_SDK NDK_HOME=$ANDROID_NDK NDK_STANDALONE=$ANDROID_TOOLCHAIN cargo apk build --target=arm-linux-androideabi