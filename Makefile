# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)

######################
# Project parameters #
######################

IMAGE_REPO := instrumentisto
IMAGE_NAME := $(strip \
	$(if $(call eq,$(image),),medea,\
	$(if $(call eq,$(image),medea-demo-edge),medea-demo,\
	$(image))))

RUST_VER := 1.53
CHROME_VERSION := 91.0
FIREFOX_VERSION := 89.0.2

CARGO_NDK_VER := 2.3.0-ndkr22b-rust$(RUST_VER)
ANDROID_TARGETS := aarch64-linux-android \
                   armv7-linux-androideabi \
                   i686-linux-android \
                   x86_64-linux-android
ANDROID_SDK_COMPILE_VERSION := $(strip \
	$(shell grep compileSdkVersion flutter/android/build.gradle \
	        | awk '{print $$2}'))
ANDROID_SDK_MIN_VERSION := $(strip \
	$(shell grep minSdkVersion flutter/android/build.gradle \
	        | awk '{print $$2}'))

crate-dir = .
ifeq ($(crate),medea-client-api-proto)
crate-dir = proto/client-api
endif
ifeq ($(crate),medea-control-api-proto)
crate-dir = proto/control-api
endif
ifeq ($(crate),medea-macro)
crate-dir = crates/medea-macro
endif
ifeq ($(crate),medea-reactive)
crate-dir = crates/medea-reactive
endif

wasm-bindgen-timeout = $(if $(call eq,$(timeout),),60,$(timeout))
webdriver-env = $(if $(call eq,$(browser),firefox),GECKO,CHROME)DRIVER_REMOTE

test.unit:
ifeq ($(browser),default)
	cd $(crate-dir)/ && \
	WASM_BINDGEN_TEST_TIMEOUT=$(wasm-bindgen-timeout) \
	cargo test --target wasm32-unknown-unknown --features mockable
else
	@make docker.up.webdriver browser=$(browser)
	sleep 10
	cd $(crate-dir)/ && \
	$(webdriver-env)="http://127.0.0.1:4444" \
	WASM_BINDGEN_TEST_TIMEOUT=$(wasm-bindgen-timeout) \
	cargo test --target wasm32-unknown-unknown --features mockable
	@make docker.down.webdriver browser=$(browser)
endif

cargo-build-crate = $(if $(call eq,$(crate),),@all,$(crate))
cargo-build-platform = $(if $(call eq,$(platform),),web,$(platform))
cargo-build-targets = $(strip \
	$(if $(call eq,$(targets),),$(ANDROID_TARGETS),$(targets)))

cargo.build:
ifeq ($(dockerized),yes)
ifeq ($(cargo-build-platform),web)
	docker run --rm --network=host -v "$(PWD)":/app -w /app \
		-u $(shell id -u):$(shell id -g) \
		-v "$(HOME)/.cargo/registry":/usr/local/cargo/registry \
		-v "$(HOME):$(HOME)" \
		-e XDG_CACHE_HOME=$(HOME) \
		ghcr.io/instrumentisto/rust:$(RUST_VER) \
			make cargo.build crate=$(cargo-build-crate) \
							 debug=$(debug) dockerized=no \
							 pre-install=yes
endif
ifeq ($(cargo-build-platform),android)
	docker run --rm --network=host -v "$(PWD)":/app -w /app \
		-u $(shell id -u):$(shell id -g) \
		-v "$(HOME)/.cargo/registry":/usr/local/cargo/registry \
		-v "$(HOME):$(HOME)" \
		-e XDG_CACHE_HOME=$(HOME) \
		instrumentisto/cargo-ndk:$(CARGO_NDK_VER) \
			make cargo.build crate=$(cargo-build-crate) \
							 debug=$(debug) dockerized=no \
							 platform=$(platform) targets=$(targets)
endif
else
ifeq ($(cargo-build-platform),web)
ifeq ($(pre-install),yes)
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
endif
	@rm -rf $(crate-dir)/pkg/
	wasm-pack build -t web $(crate-dir) \
		$(if $(call eq,$(debug),no),,--dev) \
		$(args)
endif
ifeq ($(cargo-build-platform),android)
	$(foreach target,$(subst $(comma), ,$(cargo-build-targets)),\
		$(call cargo.build.medea-jason.android,$(target),$(debug)))
endif
endif
define cargo.build.medea-jason.android
	$(eval target := $(strip $(1)))
	$(eval debug := $(strip $(2)))
	cargo ndk -p $(ANDROID_SDK_COMPILE_VERSION) -t $(target) \
	          -o flutter/android/src/main/jniLibs \
	          --manifest-path=Cargo.toml \
		build $(if $(call eq,$(debug),no),--release,) $(args)
endef

# Format Rust sources with rustfmt.
#
# Usage:
#	make cargo.fmt [check=(no|yes)]

cargo.fmt:
	cargo +nightly fmt --all $(if $(call eq,$(check),yes),-- --check,)

# Install or upgrade project's Android targets for Rust.
#
# Usage:
#	make rustup.android

rustup.android:
	rustup target add $(ANDROID_TARGETS)

####################
# Flutter commands #
####################

# Show Android SDK compile API version of medea_jason Flutter plugin.
#
# Usage:
#	make flutter.android.compile_api_version

flutter.android.version.compile:
	@printf "$(ANDROID_SDK_COMPILE_VERSION)"


# Show Android SDK minimal API version of medea_jason Flutter plugin.
#
# Usage:
#	make flutter.android.version.min

flutter.android.version.min:
	@printf "$(ANDROID_SDK_MIN_VERSION)"


# Resolve Flutter project dependencies.
#
# Usage:
#	make flutter [cmd=(pub get|<flutter-cmd>)]

flutter:
	cd flutter && \
	flutter $(if $(call eq,$(cmd),),pub get,$(cmd))


# Format Flutter Dart sources with dartfmt.
#
# Usage:
#	make flutter.fmt [check=(no|yes)]

flutter.fmt:
	flutter format $(if $(call eq,$(check),yes),-n --set-exit-if-changed,) \
		flutter/


# Lint Rust sources with Clippy.
#
# Usage:
#	make cargo.lint

cargo.lint:
	cargo clippy --workspace -- -D clippy::pedantic -D warnings
	$(foreach target,$(subst $(comma), ,$(ANDROID_TARGETS)),\
		$(call cargo.lint.medea-jason.android,$(target)))
define cargo.lint.medea-jason.android
	$(eval target := $(strip $(1)))
	cargo clippy --manifest-path Cargo.toml --target=$(target) -- \
		-D clippy::pedantic -D warnings
endef


# Lint Flutter Dart sources with dartanalyzer.
#
# Usage:
#	make flutter.lint

flutter.lint:
	flutter analyze flutter/


# Runs medea_jason Flutter plugin example app on attached device.
#
# Usage:
#	make flutter.run [debug=(yes|no)] [device=<device-id>]

flutter.run:
	cd flutter/example/ && \
	flutter run $(if $(call eq,$(debug),no),--release,) \
		$(if $(call eq,$(device),),,-d $(device))




#################
# Yarn commands #
#################

# Resolve NPM project dependencies with Yarn.
#
# Optional 'cmd' parameter may be used for handy usage of docker-wrapped Yarn,
# for example: make yarn cmd='upgrade'
#
# Usage:
#	make yarn [cmd=('install --pure-lockfile'|<yarn-cmd>)]
#	          [pkg=(e2e|medea-demo)]
#	          [dockerized=(yes|no)]

yarn-cmd = $(if $(call eq,$(cmd),),install --pure-lockfile,$(cmd))
yarn-pkg-dir = $(if $(call eq,$(pkg),medea-demo),jason/demo,jason/e2e-demo)

yarn:
ifneq ($(dockerized),no)
	docker run --rm --network=host -v "$(PWD)":/app -w /app \
	           -u $(shell id -u):$(shell id -g) \
		node:latest \
			make yarn cmd='$(yarn-cmd)' pkg=$(pkg) dockerized=no
else
	yarn --cwd=$(yarn-pkg-dir) $(yarn-cmd)
endif


# Show version of project's Yarn package.
#
# Usage:
#	make cargo.version [pkg=medea-demo]

yarn.version:
	@printf "$(strip $(shell grep -m1 '"version": "' jason/demo/package.json \
	                         | cut -d '"' -f4))"




# Runs Flutter plugin integration tests on an attached device.
#
# Usage:
#	make test.flutter [device=<device-id>]

test.flutter:
	cd flutter/example/ && \
	flutter drive --driver=test_driver/integration_test.dart \
	              --target=integration_test/jason.dart \
	              $(if $(call eq,$(device),),,-d $(device))




####################
# Waiting commands #
####################

# Waits for some port on localhost to become open.
#
# Usage:
#   make wait.port [port=<port>]

wait.port:
	while ! timeout 1 bash -c "echo > /dev/tcp/localhost/$(port)"; \
		do sleep 1; done



######################
# Releasing commands #
######################

# Build and publish project crate to crates.io.
#
# Usage:
#	make release.crates crate=(medea|medea-jason|<crate-name>)
#	                    [token=($CARGO_TOKEN|<cargo-token>)]
#	                    [publish=(no|yes)]

release-crates-token = $(if $(call eq,$(token),),${CARGO_TOKEN},$(token))

release.crates:
ifneq ($(filter $(crate),medea-jason medea-client-api-proto medea-macro medea-reactive),)
	cd $(crate-dir)/ && \
	$(if $(call eq,$(publish),yes),\
		cargo publish --token $(release-crates-token) ,\
		cargo package --allow-dirty )
endif


# Build and publish project crate to NPM.
#
# Usage:
#	make release.npm crate=medea-jason
#	                 [publish=(no|yes)]

release.npm:
ifneq ($(filter $(crate),medea-jason),)
	@make cargo.build crate=$(crate) debug=no dockerized=no
ifeq ($(publish),yes)
	wasm-pack publish $(crate-dir)/
endif
endif


# Stop dockerized WebDriver and remove all related containers.
#
# Usage:
#   make docker.down.webdriver [browser=(chrome|firefox)]

docker.down.webdriver:
	-docker stop medea-webdriver-$(if $(call eq,$(browser),),chrome,$(browser))


# Run dockerized WebDriver.
#
# Usage:
#   make docker.up.webdriver [browser=(chrome|firefox)]

docker.up.webdriver:
	-@make docker.down.webdriver browser=chrome
	-@make docker.down.webdriver browser=firefox
ifeq ($(browser),firefox)
	docker run --rm -d --network=host --shm-size 512m \
		--name medea-webdriver-firefox \
		ghcr.io/instrumentisto/geckodriver:$(FIREFOX_VERSION)
else
	docker run --rm -d --network=host \
		--name medea-webdriver-chrome \
		selenoid/chrome:$(CHROME_VERSION)
endif


# Generate project documentation of Rust sources.
#
# Usage:
#	make docs.rust [crate=(@all|medea|medea-jason|<crate-name>)]
#	               [open=(yes|no)] [clean=(no|yes)]
#	               [dev=(no|yes)]

docs-rust-crate = $(if $(call eq,$(crate),),@all,$(crate))

docs.rust:
ifeq ($(clean),yes)
	@rm -rf target/doc/
endif
	$(if $(call eq,$(docs-rust-crate),@all),\
		cargo doc --workspace,\
		cd $(crate-dir)/ && cargo doc)\
			--no-deps \
			$(if $(call eq,$(dev),yes),--document-private-items,) \
			$(if $(call eq,$(open),no),,--open)

.PHONY: flutter cargo.build