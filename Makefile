###############################
# Common defaults/definitions #
###############################

comma := ,

# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)




######################
# Project parameters #
######################

IMAGE_REPO := instrumentisto
IMAGE_NAME := $(strip \
	$(if $(call eq,$(image),medea-demo-edge),medea-demo,\
	$(or $(image),medea-control-api-mock)))

RUST_VER := 1.63
CHROME_VERSION := 102.0
FIREFOX_VERSION := 97.0.1-driver0.30.0

CARGO_NDK_VER := 2.8.0-ndkr23b-rust$(RUST_VER)
ANDROID_TARGETS := aarch64-linux-android \
                   armv7-linux-androideabi \
                   i686-linux-android \
                   x86_64-linux-android
ANDROID_SDK_COMPILE_VERSION = $(strip \
	$(shell grep compileSdkVersion flutter/android/build.gradle \
	        | awk '{print $$2}'))
ANDROID_SDK_MIN_VERSION = $(strip \
	$(shell grep minSdkVersion flutter/android/build.gradle \
	        | awk '{print $$2}'))
LINUX_TARGETS := x86_64-unknown-linux-gnu
MACOS_TARGETS := x86_64-apple-darwin
WEB_TARGETS := wasm32-unknown-unknown
WINDOWS_TARGETS := x86_64-pc-windows-msvc

crate-dir = .
ifeq ($(crate),medea-client-api-proto)
crate-dir = proto/client-api
endif
ifeq ($(crate),medea-control-api-proto)
crate-dir = proto/control-api
endif
ifeq ($(crate),medea-control-api-mock)
crate-dir = mock/control-api
endif
ifeq ($(crate),medea-macro)
crate-dir = crates/medea-macro
endif
ifeq ($(crate),medea-reactive)
crate-dir = crates/medea-reactive
endif
crate-ver := $(strip \
	$(shell grep -m1 'version = "' $(crate-dir)/Cargo.toml | cut -d '"' -f2))




###########
# Aliases #
###########

# Build all project executables.
#
# Usage:
#	make build

build: build.jason


build.jason: cargo.build.jason


# Resolve all project dependencies.
#
# Usage:
#	make deps

deps: cargo flutter yarn


docs: docs.rust


down: down.dev


fmt: cargo.fmt flutter.fmt


lint: cargo.lint flutter.lint


# Build and publish project crate everywhere.
#
# Usage:
#	make release crate=(medea-jason|<crate-name>)
#	             [publish=(no|yes)]

release: release.crates release.npm


# Run all project tests.
#
# Usage:
#	make test

test:
	@make test.unit
	@make test.e2e up=yes dockerized=no


up: up.dev




####################
# Running commands #
####################

# Stop non-dockerized Control API mock server.
#
# Usage:
#   make down.control

down.control:
	-killall medea-control-api-mock


down.coturn: docker.down.coturn


down.demo: docker.down.demo


# Stop all processes in Medea and Jason development environment.
#
# Usage:
#	make down.dev

down.dev:
	@make docker.down.medea
	@make down.control
	@make docker.down.coturn


down.medea: docker.down.medea


# Run Control API mock server.
#
# Usage:
#  make up.control [background=(no|yes)]

up.control:
	cargo build -p medea-control-api-mock
	make wait.port port=6565
	cargo run -p medea-control-api-mock $(if $(call eq,$(background),yes),&,)


up.coturn: docker.up.coturn


up.demo: docker.up.demo


# Run Medea and Jason development environment.
#
# Usage:
#	make up.dev

up.dev: up.coturn
	$(MAKE) -j3 up.jason docker.up.medea up.control


up.medea: docker.up.medea


# Run Jason E2E demo in development mode.
#
# Usage:
#	make up.jason

up.jason:
	npm run start --prefix=./e2e-demo




##################
# Cargo commands #
##################

# Resolve Cargo project dependencies.
#
# Usage:
#	make cargo [cmd=(fetch|<cargo-cmd>)]

cargo:
	cargo $(or $(cmd),fetch)


# Build `medea-jason` crate.
#
# Usage:
#	make cargo.build.jason [args=<cargo-build-args>]
#		[( [platform=web [targets=($(WEB_TARGETS)|<t1>[,<t2>...])]]
#		 | platform=all
#		 | platform=android [targets=($(ANDROID_TARGETS)|<t1>[,<t2>...])]
#		 | platform=linux [targets=($(LINUX_TARGETS)|<t1>[,<t2>...])]
#		 | platform=macos [targets=($(MACOS_TARGETS)|<t1>[,<t2>...])]
#		 | platform=windows [targets=($(WINDOWS_TARGETS)|<t1>[,<t2>...])] )]
#		[debug=(yes|no)] [dockerized=(no|yes)]

cargo-build-platform = $(or $(platform),web)
cargo-build-targets-android = $(or $(targets),$(ANDROID_TARGETS))
cargo-build-targets-linux = $(or $(targets),$(LINUX_TARGETS))
cargo-build-targets-macos = $(or $(targets),$(MACOS_TARGETS))
cargo-build-targets-web = $(or $(targets),$(WEB_TARGETS))
cargo-build-targets-windows = $(or $(targets),$(WINDOWS_TARGETS))

cargo.build.jason:
ifeq ($(platform),all)
	@make cargo.build.jason platform=android
	@make cargo.build.jason platform=linux
	@make cargo.build.jason platform=web
	@make cargo.build.jason platform=windows
else
ifeq ($(dockerized),yes)
ifeq ($(cargo-build-platform),web)
	docker run --rm --network=host -v "$(PWD)":/app -w /app \
		-u $(shell id -u):$(shell id -g) \
		-v "$(HOME)/.cargo/registry":/usr/local/cargo/registry \
		-v "$(HOME):$(HOME)" \
		-e XDG_CACHE_HOME=$(HOME) \
		ghcr.io/instrumentisto/rust:$(RUST_VER) \
			make cargo.build.jason debug=$(debug) dockerized=no \
			                       platform=web args="$(args)" \
			                       targets=$(targets) \
			                       pre-install=yes
endif
ifeq ($(cargo-build-platform),android)
	docker run --rm --network=host -v "$(PWD)":/app -w /app \
		-u $(shell id -u):$(shell id -g) \
		-v "$(HOME)/.cargo/registry":/usr/local/cargo/registry \
		-v "$(HOME):$(HOME)" \
		-e XDG_CACHE_HOME=$(HOME) \
		ghcr.io/instrumentisto/cargo-ndk:$(CARGO_NDK_VER) \
			make cargo.build.jason debug=$(debug) dockerized=no \
			                       platform=android args="$(args)"
			                       targets=$(targets)
endif
else
ifeq ($(cargo-build-platform),web)
ifeq ($(pre-install),yes)
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
endif
	@rm -rf ./pkg/
	wasm-pack build -t web ./ $(if $(call eq,$(debug),no),,--dev) $(args)
endif
ifeq ($(cargo-build-platform),android)
	$(foreach target,$(subst $(comma), ,$(cargo-build-targets-android)),\
		$(call cargo.build.medea-jason.android,$(target),$(debug)))
endif
ifeq ($(cargo-build-platform),linux)
	$(foreach target,$(subst $(comma), ,$(cargo-build-targets-linux)),\
		$(call cargo.build.medea-jason.linux,$(target),$(debug)))
endif
ifeq ($(cargo-build-platform),macos)
	$(foreach target,$(subst $(comma), ,$(cargo-build-targets-macos)),\
		$(call cargo.build.medea-jason.macos,$(target),$(debug)))
endif
ifeq ($(cargo-build-platform),windows)
	$(foreach target,$(subst $(comma), ,$(cargo-build-targets-windows)),\
		$(call cargo.build.medea-jason.windows,$(target),$(debug)))
endif
endif
endif
define cargo.build.medea-jason.android
	$(eval target := $(strip $(1)))
	$(eval debug := $(strip $(2)))
	cargo ndk -p $(ANDROID_SDK_MIN_VERSION) -t $(target) \
	          -o ./flutter/android/src/main/jniLibs \
	          --manifest-path=./Cargo.toml \
		build $(if $(call eq,$(debug),no),--release,) $(args)
endef
define cargo.build.medea-jason.linux
	$(eval target := $(strip $(1)))
	$(eval debug := $(strip $(2)))
	cargo build --target $(target) $(if $(call eq,$(debug),no),--release,) \
	            --manifest-path=./Cargo.toml \
	            $(args)
	@mkdir -p ./flutter/linux/lib/$(target)/
	cp -f target/$(target)/$(if $(call eq,$(debug),no),release,debug)/libmedea_jason.so \
	      ./flutter/linux/lib/$(target)/libmedea_jason.so
endef
define cargo.build.medea-jason.macos
	$(eval target := $(strip $(1)))
	$(eval debug := $(strip $(2)))
	cargo build --target $(target) $(if $(call eq,$(debug),no),--release,) \
	            --manifest-path=./Cargo.toml \
	            $(args)
	@mkdir -p ./flutter/macos/lib/$(target)/
	cp -f target/$(target)/$(if $(call eq,$(debug),no),release,debug)/libmedea_jason.dylib \
	      ./flutter/macos/lib/$(target)/libmedea_jason.dylib
endef
define cargo.build.medea-jason.windows
	$(eval target := $(strip $(1)))
	$(eval debug := $(strip $(2)))
	cargo build --target $(target) $(if $(call eq,$(debug),no),--release,) \
	            --manifest-path=./Cargo.toml \
	            $(args)
	@mkdir -p ./flutter/windows/lib/$(target)/
	cp -f target/$(target)/$(if $(call eq,$(debug),no),release,debug)/medea_jason.dll \
	      ./flutter/windows/lib/$(target)/medea_jason.dll
endef


# Show permalink to CHANGELOG of a concrete version of project's Cargo crate.
#
# Usage:
#	make cargo.changelog.link [crate=(medea-jason|<crate-name>)]
#	                          [ver=($(crate-ver)|<version>)]

cargo-changelog-link-ver = $(if $(call eq,$(ver),),$(crate-ver),$(ver))

cargo.changelog.link:
	@printf "https://github.com/instrumentisto/medea-jason/blob/$(or $(crate),medea-jason)-$(cargo-changelog-link-ver)/$(if $(call eq,$(crate-dir),.),,$(crate-dir)/)CHANGELOG.md#$(shell sed -n '/^## \[$(cargo-changelog-link-ver)\]/{s/^## \[\(.*\)\][^0-9]*\([0-9].*\)/\1--\2/;s/[^0-9a-z-]*//g;p;}' $(crate-dir)/CHANGELOG.md)"


# Format Rust sources with rustfmt.
#
# Usage:
#	make cargo.fmt [check=(no|yes)]

cargo.fmt:
	cargo +nightly fmt --all $(if $(call eq,$(check),yes),-- --check,)


# Generate sources using Cargo.
#
# Usage:
#	make cargo.gen [( crate=medea-control-api-proto
#	                | crate=medea-jason [dockerized=(no|yes) )]

cargo.gen:
ifeq ($(crate),medea-control-api-proto)
	@rm -rf $(crate-dir)/src/grpc/api.rs \
	        $(crate-dir)/src/grpc/callback.rs
	cargo build -p $(crate) --all-features
endif
ifeq ($(crate),medea-jason)
	cargo clean -p $(crate)
	make cargo.build.jason platform=android args="--features dart-codegen" \
	     dockerized=$(dockerized)
	make flutter.fmt
endif


# Lint Rust sources with Clippy.
#
# Usage:
#	make cargo.lint

cargo.lint:
	cargo clippy --workspace --all-features -- -D warnings
	$(foreach target,$(subst $(comma), ,\
		$(ANDROID_TARGETS) $(LINUX_TARGETS) $(MACOS_TARGETS) $(WEB_TARGETS) \
		$(WINDOWS_TARGETS)),\
			$(call cargo.lint.medea-jason,$(target)))
define cargo.lint.medea-jason
	$(eval target := $(strip $(1)))
	cargo clippy --manifest-path Cargo.toml --target=$(target) -- -D warnings
endef


# Show version of project's Cargo crate.
#
# Usage:
#	make cargo.version [crate=(medea-jason|<crate-name>)]

cargo.version:
	@printf "$(crate-ver)"


# Install or upgrade all the required project's targets for Rust.
#
# Usage:
#	make rustup.targets [only=(android|linux|web|windows)]

rustup-targets = $(ANDROID_TARGETS) \
                 $(LINUX_TARGETS) \
                 $(MACOS_TARGETS) \
                 $(WEB_TARGETS) \
                 $(WINDOWS_TARGETS)
ifeq ($(only),android)
rustup-targets = $(ANDROID_TARGETS)
endif
ifeq ($(only),linux)
rustup-targets = $(LINUX_TARGETS)
endif
ifeq ($(only),macos)
rustup-targets = $(MACOS_TARGETS)
endif
ifeq ($(only),web)
rustup-targets = $(WEB_TARGETS)
endif
ifeq ($(only),windows)
rustup-targets = $(WINDOWS_TARGETS)
endif

rustup.targets:
	rustup target add $(rustup-targets)




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
	flutter $(or $(cmd),pub get)


# Format Flutter Dart sources with dartfmt.
#
# Usage:
#	make flutter.fmt [check=(no|yes)]

flutter.fmt:
	flutter format $(if $(call eq,$(check),yes),-n --set-exit-if-changed,) \
		flutter/
ifeq ($(wildcard flutter/.packages),)
	@make flutter cmd='pub get'
endif
	@make flutter cmd='pub run import_sorter:main --no-comments \
	                   $(if $(call eq,$(check),yes),--exit-if-changed,)'


# Lint Flutter Dart sources with dartanalyzer.
#
# Usage:
#	make flutter.lint

flutter.lint:
ifeq ($(wildcard flutter/test/e2e/suite.g.dart),)
	@make flutter.gen overwrite=yes dockerized=$(dockerized)
endif
	flutter analyze flutter/


# Runs medea_jason Flutter plugin example app on attached device.
#
# Usage:
#	make flutter.run [debug=(yes|no)] [device=<device-id>]

flutter.run:
	cd flutter/example/ && \
	flutter run $(if $(call eq,$(debug),no),--release,) \
		$(if $(call eq,$(device),),,-d $(device))


# Generates assets required for Flutter Web Jason plugin.
#
# Usage:
#	make flutter.web.assets

flutter.web.assets:
	@rm -rf flutter/assets/pkg
	wasm-pack build -d flutter/assets/pkg --no-typescript -t web
	rm -rf flutter/assets/pkg/*.md \
	       flutter/assets/pkg/.gitignore \
	       flutter/assets/pkg/package.json




# Run `build_runner` Flutter tool to generate project Dart sources.
#
# Usage:
#	make flutter.gen [overwrite=(yes|no)]

flutter.gen:
ifeq ($(wildcard flutter/pubspec.lock),)
	@make flutter
endif
	cd flutter && \
	flutter pub run build_runner build \
		$(if $(call eq,$(overwrite),no),,--delete-conflicting-outputs)




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

yarn-cmd = $(or $(cmd),install --pure-lockfile)
yarn-pkg-dir = $(if $(call eq,$(pkg),medea-demo),demo,e2e-demo)

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
	@printf "$(strip $(shell grep -m1 '"version": "' demo/package.json \
	                         | cut -d '"' -f4))"




##########################
# Documentation commands #
##########################

# Generate project documentation of Rust sources.
#
# Usage:
#	make docs.rust [crate=(@all|medea-jason|<crate-name>)]
#	               [open=(yes|no)] [clean=(no|yes)]
#	               [dev=(no|yes)]

docs.rust:
ifeq ($(clean),yes)
	@rm -rf target/doc/
endif
	$(if $(call eq,$(or $(crate),@all),@all),\
		cargo doc --workspace,\
		cd $(crate-dir)/ && cargo doc)\
			--no-deps \
			$(if $(call eq,$(dev),yes),--document-private-items,) \
			$(if $(call eq,$(open),no),,--open)




####################
# Testing commands #
####################

# Run Rust unit tests of project.
#
# Usage:
#	make test.unit [( [crate=@all]
#	                | crate=<crate-name> [features=(all|<f1>[,<f2>...])]
#	                | crate=medea-jason
#	                  [browser=(chrome|firefox|default)]
#	                  [timeout=(60|<seconds>)] )]

webdriver-env = $(if $(call eq,$(browser),firefox),GECKO,CHROME)DRIVER_REMOTE

test.unit:
ifeq ($(or $(crate),@all),@all)
	@make test.unit crate=medea-macro
	@make test.unit crate=medea-reactive
	@make test.unit crate=medea-client-api-proto
	@make test.unit crate=medea-control-api-proto
	@make test.unit crate=medea-jason
else
ifeq ($(crate),medea-jason)
ifeq ($(browser),default)
	cd $(crate-dir)/ && \
	WASM_BINDGEN_TEST_TIMEOUT=$(or $(timeout),60) \
	cargo test --target wasm32-unknown-unknown --features mockable
else
	@make docker.up.webdriver browser=$(browser)
	sleep 10
	cd $(crate-dir)/ && \
	$(webdriver-env)="http://127.0.0.1:4444" \
	WASM_BINDGEN_TEST_TIMEOUT=$(or $(timeout),60) \
	cargo test --target wasm32-unknown-unknown --features mockable
	@make docker.down.webdriver browser=$(browser)
endif
else
	cargo test -p $(crate) $(if $(call eq,$(or $(features),all),all),\
		--all-features ,\
		--features $(features) )
endif
endif


# Run E2E tests of project.
#
# Usage:
#	make test.e2e [(only=<regex>|only-tags=<tag-expression>)]
#		[( [up=no]
#		 | up=yes [browser=(chrome|firefox)]
#		          [( [dockerized=no]
#		           | dockerized=yes [tag=(dev|<tag>)] [rebuild=(no|yes)] )]
#		          [debug=(yes|no)]
#		          [( [background=no]
#		           | background=yes [log=(no|yes)] )]

test.e2e:
ifeq ($(up),yes)
ifeq ($(dockerized),yes)
ifeq ($(rebuild),yes)
	@make docker.build image=medea-control-api-mock debug=$(debug) tag=$(tag)
endif
endif
	@make docker.up.e2e browser=$(browser) background=yes log=$(log) \
	                    dockerized=$(dockerized) tag=$(tag) debug=$(debug)
	@make wait.port port=4444
endif
	cargo test -p medea-e2e --test e2e \
		$(if $(call eq,$(only),),\
			$(if $(call eq,$(only-tags),),,-- --tags '$(only-tags)'),\
			-- --name '$(only)')
ifeq ($(up),yes)
	@make docker.down.e2e
endif

# Run E2E desktop tests of project.
#
# Usage:
#	make test.e2e.desktop [(only=<regex>|only-tags=<tag-expression>)]
# 		[device=<device-id>]
#		[( [up=no] | up=yes
#		          [( [dockerized=no]
#		           | dockerized=yes [tag=(dev|<tag>)] [rebuild=(no|yes)] )]
#		          [debug=(yes|no)]
#		          [( [background=no]
#		           | background=yes [log=(no|yes)] )]

test.e2e.desktop:
ifeq ($(up),yes)
ifeq ($(dockerized),yes)
ifeq ($(rebuild),yes)
	@make docker.build image=medea-control-api-mock debug=$(debug) tag=$(tag)
endif
endif
	@make docker.up.e2e background=yes log=$(log) \
	                    dockerized=$(dockerized) tag=$(tag) debug=$(debug)
endif
ifeq ($(wildcard flutter/test/e2e/suite.g.dart),)
	@make flutter.gen overwrite=yes dockerized=$(dockerized)
endif
	cd flutter/example/ && \
	flutter drive --driver=test_driver/integration_test.dart \
		--target=../test/e2e/suite.dart $(if $(call eq,$(device),),,-d $(device))
ifeq ($(up),yes)
	@make docker.down.e2e
endif

# Run E2E windows tests of project in vagrant vm.
#
# Usage:
#	make test.e2e.desktop.windows [(only=<regex>|only-tags=<tag-expression>)]
#		[( [up=no] | up=yes
#		          [( [dockerized=no]
#		           | dockerized=yes [tag=(dev|<tag>)] [rebuild=(no|yes)] )]
#		          [debug=(yes|no)]
#		          [( [background=no]
#		           | background=yes [log=(no|yes)] )]

test.e2e.desktop.windows:
ifeq ($(up),yes)
ifeq ($(dockerized),yes)
ifeq ($(rebuild),yes)
	@make docker.build image=medea-control-api-mock debug=$(debug) tag=$(tag)
endif
endif
	env $(docker-up-e2e-env) \
	docker-compose -f e2e/docker-compose$(if $(call eq,$(dockerized),yes),,.host).yml \
		up $(if $(call eq,$(dockerized),yes),\
		   $(if $(call eq,$(background),yes),-d,--abort-on-container-exit),-d)
endif
	cd windows_test_staff/ && \
	vagrant up
ifeq ($(up),yes)
	@make docker.down.e2e
endif

# Build flutter e2e test as bundle.
#
# Usage:
#	make test.e2e.desktop.windows.build
test.e2e.desktop.windows.build:
	cd flutter/example/ && \
	flutter build windows --target=../test/e2e/suite.dart --debug

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
#	make release.crates crate=(medea-jason|<crate-name>)
#	                    [token=($CARGO_TOKEN|<cargo-token>)]
#	                    [publish=(no|yes)]

release.crates:
ifneq ($(filter $(crate),medea-jason medea-client-api-proto medea-control-api-proto medea-macro medea-reactive),)
	cd $(crate-dir)/ && \
	$(if $(call eq,$(publish),yes),\
		cargo publish --token $(or $(token),${CARGO_TOKEN}) ,\
		cargo package --allow-dirty )
endif


release.helm: helm.package.release


# Build and publish project crate to NPM.
#
# Usage:
#	make release.npm crate=medea-jason
#	                 [publish=(no|yes)]

release.npm:
ifneq ($(filter $(crate),medea-jason),)
	@make cargo.build debug=no dockerized=no
ifeq ($(publish),yes)
	wasm-pack publish $(crate-dir)/
endif
endif




###################
# Docker commands #
###################

docker-env = $(strip $(if $(call eq,$(minikube),yes),\
	$(subst export,,$(shell minikube docker-env | cut -d '\#' -f1)),))

# Build project Docker image with a given tag.
#
# Usage:
#	make docker.build [debug=(yes|no)] [no-cache=(no|yes)]
#		[image=(medea-control-api-mock|medea-demo|medea-demo-edge)]
#		[tag=(dev|<tag>)]
#		[minikube=(no|yes)]

docker-build-tag = $(if $(call eq,$(tag),),dev,$(tag))
docker-build-dir = .
ifeq ($(image),medea-demo)
docker-build-dir = demo
endif
docker-build-file = $(docker-build-dir)/Dockerfile
ifeq ($(image),medea-control-api-mock)
docker-build-file = mock/control-api/Dockerfile
endif
ifeq ($(image),medea-demo-edge)
docker-build-file = Dockerfile
endif

docker.build:
	$(docker-env) \
	docker build $(if $(call eq,$(minikube),yes),,--network=host) --force-rm \
		$(if $(call eq,$(no-cache),yes),\
			--no-cache --pull,) \
		--build-arg rust_ver=$(RUST_VER) \
		--build-arg rustc_mode=$(if $(call eq,$(debug),no),release,debug) \
		--build-arg rustc_opts=$(if $(call eq,$(debug),no),--release,) \
		--build-arg debug=$(if $(call eq,$(debug),no),no,yes) \
		-t $(IMAGE_REPO)/$(IMAGE_NAME):$(docker-build-tag) \
		-f $(docker-build-file) $(docker-build-dir)/


# Stop dockerized Control API mock server and remove all related containers.
#
# Usage:
#	make docker.down.control

docker.down.control:
	-docker stop medea-control-api-mock


# Stop Coturn STUN/TURN server in Docker Compose environment
# and remove all related containers.
#
# Usage:
#	make docker.down.coturn

docker.down.coturn:
	docker-compose -f docker-compose.coturn.yml down --rmi=local -v


# Stop demo application in Docker Compose environment
# and remove all related containers.
#
# Usage:
#	make docker.down.demo

docker.down.demo:
	docker-compose -f demo/docker-compose.yml down --rmi=local -v


# Stop E2E tests environment in Docker Compose and remove all related
# containers.
#
# Usage:
#	make docker.down.e2e

docker.down.e2e: down.control
	@make docker.down.medea
	docker-compose -f e2e/docker-compose.yml down --rmi=local -v


# Stop Medea media server in Docker Compose environment
# and remove all related containers.
#
# Usage:
# 	make docker.down.medea

docker.down.medea:
	docker-compose -f docker-compose.medea.yml down --rmi=local -v


# Stop dockerized WebDriver and remove all related containers.
#
# Usage:
#   make docker.down.webdriver [browser=(chrome|firefox)]

docker.down.webdriver:
	-docker stop medea-webdriver-$(if $(call eq,$(browser),),chrome,$(browser))


# Pull project Docker images from Container Registry.
#
# Usage:
#	make docker.pull
#		[image=(medea-control-api-mock|medea-demo)]
#		[repos=($(IMAGE_REPO)|<prefix-1>[,<prefix-2>...])]
#		[tags=(@all|<t1>[,<t2>...])]
#		[minikube=(no|yes)]

docker-pull-repos = $(or $(repos),$(IMAGE_REPO))
docker-pull-tags = $(or $(tags),@all)

docker.pull:
ifeq ($(docker-pull-tags),@all)
	$(foreach repo,$(subst $(comma), ,$(docker-pull-repos)),\
		$(call docker.pull.do,$(repo)/$(IMAGE_NAME) --all-tags))
else
	$(foreach tag,$(subst $(comma), ,$(docker-pull-tags)),\
		$(foreach repo,$(subst $(comma), ,$(docker-pull-repos)),\
			$(call docker.pull.do,$(repo)/$(IMAGE_NAME):$(tag))))
endif
define docker.pull.do
	$(eval image-full := $(strip $(1)))
	$(docker-env) \
	docker pull $(image-full)
endef


# Push project Docker images to Container Registry.
#
# Usage:
#	make docker.push
#		[image=(medea-control-api-mock|medea-demo)]
#		[repos=($(IMAGE_REPO)|<prefix-1>[,<prefix-2>...])]
#		[tags=(dev|<t1>[,<t2>...])]
#		[minikube=(no|yes)]

docker-push-repos = $(or $(repos),$(IMAGE_REPO))
docker-push-tags = $(or $(tags),dev)

docker.push:
	$(foreach tag,$(subst $(comma), ,$(docker-push-tags)),\
		$(foreach repo,$(subst $(comma), ,$(docker-push-repos)),\
			$(call docker.push.do,$(repo)/$(IMAGE_NAME):$(tag))))
define docker.push.do
	$(eval image-full := $(strip $(1)))
	$(docker-env) \
	docker push $(image-full)
endef


# Tag project Docker image with given tags.
#
# Usage:
#	make docker.tag [of=(dev|<tag>)]
#		[image=(medea-control-api-mock|medea-demo)]
#		[repos=($(IMAGE_REPO)|<with-prefix-1>[,<with-prefix-2>...])]
#		[tags=(dev|<with-t1>[,<with-t2>...])]
#		[minikube=(no|yes)]

docker-tag-of := $(or $(of),dev)
docker-tag-with := $(or $(tags),dev)
docker-tag-repos = $(or $(repos),$(IMAGE_REPO))

docker.tag:
	$(foreach tag,$(subst $(comma), ,$(docker-tag-with)),\
		$(foreach repo,$(subst $(comma), ,$(docker-tag-repos)),\
			$(call docker.tag.do,$(repo),$(tag))))
define docker.tag.do
	$(eval repo := $(strip $(1)))
	$(eval tag := $(strip $(2)))
	$(docker-env) \
	docker tag $(IMAGE_REPO)/$(IMAGE_NAME):$(if $(call eq,$(of),),dev,$(of)) \
	           $(repo)/$(IMAGE_NAME):$(tag)
endef


# Save project Docker images to a tarball file.
#
# Usage:
#	make docker.tar [to-file=(.cache/image.tar|<file-path>)]
#		[image=(medea-control-api-mock|medea-demo)]
#		[tags=(dev|<t1>[,<t2>...])]
#		[minikube=(no|yes)]

docker-tar-file = $(or $(to-file),.cache/image.tar)
docker-tar-tags = $(or $(tags),dev)

docker.tar:
	@mkdir -p $(dir $(docker-tar-file))
	$(docker-env) \
	docker save -o $(docker-tar-file) \
		$(foreach tag,$(subst $(comma), ,$(docker-tar-tags)),\
			$(IMAGE_REPO)/$(IMAGE_NAME):$(tag))


# Load project Docker images from a tarball file.
#
# Usage:
#	make docker.untar [from-file=(.cache/image.tar|<file-path>)]
#		[minikube=(no|yes)]

docker.untar:
	$(docker-env) \
	docker load -i $(or $(from-file),.cache/image.tar)


# Run dockerized Medea Control API mock server.
#
# Usage:
#   make docker.up.control [tag=(dev|<docker-tag>)]

docker.up.control:
	docker run --rm -d --network=host \
		--name medea-control-api-mock \
		$(IMAGE_REPO)/medea-control-api-mock:$(or $(tag),dev)


# Run Coturn STUN/TURN server in Docker Compose environment.
#
# Usage:
#	make docker.up.coturn [background=(yes|no)]

docker.up.coturn: docker.down.coturn
	docker-compose -f docker-compose.coturn.yml up \
		$(if $(call eq,$(background),no),--abort-on-container-exit,-d)


# Run demo application in Docker Compose environment.
#
# Usage:
#	make docker.up.demo

docker.up.demo: docker.down.demo
	docker-compose -f demo/docker-compose.yml up


# Run E2E tests environment in Docker Compose.
#
# Usage:
#	make docker.up.e2e [browser=(chrome|firefox)]
#	                   [( [dockerized=no]
#	                    | dockerized=yes [medea-tag=(dev|<tag>)]
#                         [control-tag=(dev|<tag>)] )]
#	                   [debug=(yes|no)]
#	                   [( [background=no]
#	                    | background=yes [log=(no|yes)])]

docker-up-e2e-env = RUST_BACKTRACE=1 \
	$(if $(call eq,$(log),yes),,RUST_LOG=warn) \
	COMPOSE_MEDEA_IMAGE_NAME=hub.instrumentisto.com/streaming/medea$(if \
		$(call eq,$(medea-tag),edge),,/review) \
	COMPOSE_MEDEA_IMAGE_VER=$(or $(medea-tag),dev) \
	COMPOSE_CONTROL_MOCK_IMAGE_VER=$(or $(control-tag),dev) \
	COMPOSE_WEBDRIVER_IMAGE_NAME=$(strip \
		$(if $(call eq,$(browser),firefox),\
			ghcr.io/instrumentisto/geckodriver ,\
			selenoid/chrome )) \
	COMPOSE_WEBDRIVER_IMAGE_VER=$(strip \
		$(if $(call eq,$(browser),firefox),\
			$(FIREFOX_VERSION) ,\
			$(CHROME_VERSION) )) \
	COMPOSE_WEBDRIVER_ENTRYPOINT=$(strip \
		$(if $(call eq,$(browser),firefox),\
			"geckodriver --binary=/opt/firefox/firefox" ,\
			/entrypoint.sh ))

docker.up.e2e: docker.down.e2e
	@make build.jason target=web debug=$(debug) dockerized=no
	env $(docker-up-e2e-env) \
	docker-compose -f e2e/docker-compose$(if $(call eq,$(dockerized),yes),,.host).yml \
		up $(if $(call eq,$(dockerized),yes),\
		   $(if $(call eq,$(background),yes),-d,--abort-on-container-exit),-d)
ifeq ($(background),yes)
ifeq ($(log),yes)
	env $(docker-up-e2e-env) \
	docker-compose -f e2e/docker-compose$(if $(call eq,$(dockerized),yes),,.host).yml \
		logs -f &
endif
endif


# Run Medea media server in Docker Compose environment.
#
# Usage:
#	make docker.up.medea [( [dockerized=no] [debug=(yes|no)]
#	                                        [background=(no|yes)]
#	                      | dockerized=yes [tag=(dev|<docker-tag>)]
#	                                       [( [background=no]
#	                                        | background=yes [log=(no|yes)] )])]
#	                     [log-to-file=(no|yes)]

docker-up-medea-image = hub.instrumentisto.com/streaming/medea
docker-up-medea-tag = $(if $(call eq,$(tag),),edge,$(tag))

docker.up.medea: docker.down.medea
	COMPOSE_MEDEA_IMAGE_NAME=$(docker-up-medea-image) \
	COMPOSE_MEDEA_IMAGE_VER=$(docker-up-medea-tag) \
	docker-compose -f docker-compose.medea.yml up \
		$(if $(call eq,$(background),yes),-d,--abort-on-container-exit)
ifeq ($(background),yes)
ifeq ($(log),yes)
	docker-compose -f docker-compose.medea.yml logs -f &
endif
endif


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
		ghcr.io/instrumentisto/geckodriver:$(FIREFOX_VERSION) \
			--binary=/opt/firefox/firefox
else
	docker run --rm -d --network=host --shm-size 512m \
		--name medea-webdriver-chrome \
		selenoid/chrome:$(CHROME_VERSION)
endif




##############################
# Helm and Minikube commands #
##############################

helm-cluster = $(or $(cluster),minikube)
helm-cluster-args = --kube-context=$(helm-cluster)

helm-chart = $(or $(chart),medea-demo)
helm-chart-dir = demo/chart/medea-demo
helm-chart-vals-dir = demo

helm-release = $(if $(call eq,$(release),),,$(release)-)$(helm-chart)
helm-release-namespace = $(strip \
	$(if $(call eq,$(helm-cluster),staging),staging,default))

# Run Helm command in context of concrete Kubernetes cluster.
#
# Usage:
#	make helm [cmd=(--help|'<command>')]
#	          [cluster=(minikube|staging)]

helm:
	helm $(helm-cluster-args) $(or $(cmd),--help)


# Show root directory path of project Helm chart.
#
# Usage:
#	make helm.dir [chart=medea-demo]

helm.dir:
	@printf "$(helm-chart-dir)"


# Remove Helm release of project Helm chart from Kubernetes cluster.
#
# Usage:
#	make helm.down [chart=medea-demo] [release=<release-name>]
#	               [cluster=(minikube|staging)]
#	               [check=(no|yes)]

helm.down:
ifeq ($(check),yes)
	$(if $(shell helm $(helm-cluster-args) list | grep '$(helm-release)'),\
		helm $(helm-cluster-args) uninstall $(helm-release) ,\
		@echo "--> No $(helm-release) release found in $(helm-cluster) cluster")
else
	helm $(helm-cluster-args) uninstall $(helm-release)
endif


# Lint project Helm chart.
#
# Usage:
#	make helm.lint [chart=medea-demo]

helm.lint:
	helm lint $(helm-chart-dir)/


# List all Helm releases in Kubernetes cluster.
#
# Usage:
#	make helm.list [cluster=(minikube|staging)]

helm.list:
	helm $(helm-cluster-args) list


# Build Helm package from project Helm chart.
#
# Usage:
#	make helm.package [chart=medea-demo]

helm-package-dir = .cache/helm/packages

helm.package:
	@rm -rf $(helm-package-dir)
	@mkdir -p $(helm-package-dir)/
	helm package --destination=$(helm-package-dir)/ $(helm-chart-dir)/


# Build and publish project Helm package to GitHub Pages.
#
# Usage:
#	make helm.package.release [chart=medea-demo] [build=(yes|no)]

helm-package-release-ver = $(strip $(shell \
	grep 'version: ' demo/chart/medea-demo/Chart.yaml | cut -d ':' -f2))

helm.package.release:
ifneq ($(build),no)
	@make helm.package chart=$(helm-chart)
endif
	git fetch origin gh-pages:gh-pages
	git checkout gh-pages
	git reset --hard
	@mkdir -p charts/
	cp -rf $(helm-package-dir)/* charts/
	if [ -n "$$(git add -v charts/)" ]; then \
		helm repo index charts/ \
			--url=https://instrumentisto.github.io/medea-jason/charts ; \
		git add -v charts/ ; \
		git commit -m \
			"Release $(helm-chart)-$(helm-package-release-ver) Helm chart" ; \
	fi
	git checkout -
	git push origin gh-pages


# Run project Helm chart in Kubernetes cluster as Helm release.
#
# Usage:
#	make helm.up [chart=medea-demo] [release=<release-name>]
#	             [force=(no|yes)]
#	             [( [atomic=no] [wait=(yes|no)]
#	              | atomic=yes )]
#	             [( [cluster=minikube] [( [rebuild=no]
#	                                    | rebuild=yes [no-cache=(no|yes)] )]
#	              | cluster=staging )]

helm.up:
ifeq ($(wildcard $(helm-chart-vals-dir)/my.$(helm-cluster).vals.yaml),)
	touch $(helm-chart-vals-dir)/my.$(helm-cluster).vals.yaml
endif
ifeq ($(helm-cluster),minikube)
ifeq ($(helm-chart),medea-demo)
ifeq ($(rebuild),yes)
	@make docker.build image=medea-demo-edge tag=dev \
	                   minikube=yes no-cache=$(no-cache)
	@make docker.build image=medea-control-api-mock tag=dev \
	                   minikube=yes no-cache=$(no-cache)
endif
endif
endif
	helm $(helm-cluster-args) upgrade --install \
		$(helm-release) $(helm-chart-dir)/ \
			--namespace=$(helm-release-namespace) \
			--values=$(helm-chart-vals-dir)/$(helm-cluster).vals.yaml \
			--values=$(helm-chart-vals-dir)/my.$(helm-cluster).vals.yaml \
			--set server.deployment.revision=$(shell date +%s) \
			--set web-client.deployment.revision=$(shell date +%s) \
			$(if $(call eq,$(force),yes),\
				--force,)\
			$(if $(call eq,$(atomic),yes),\
				--atomic,\
			$(if $(call eq,$(wait),no),,\
				--wait ))


# Bootstrap Minikube cluster (local Kubernetes) for development environment.
#
# The bootsrap script is updated automatically to the latest version every day.
# For manual update use 'update=yes' command option.
#
# Usage:
#	make minikube.boot [update=(no|yes)]
#	                   [driver=(virtualbox|hyperkit|hyperv)]
#	                   [k8s-version=<kubernetes-version>]

minikube.boot:
ifeq ($(update),yes)
	$(call minikube.boot.download)
else
ifeq ($(wildcard $(HOME)/.minikube/bootstrap.sh),)
	$(call minikube.boot.download)
else
ifneq ($(shell find $(HOME)/.minikube/bootstrap.sh -mmin +1440),)
	$(call minikube.boot.download)
endif
endif
endif
	@$(if $(cal eq,$(driver),),,MINIKUBE_VM_DRIVER=$(driver)) \
	 $(if $(cal eq,$(k8s-version),),,MINIKUBE_K8S_VER=$(k8s-version)) \
		$(HOME)/.minikube/bootstrap.sh
define minikube.boot.download
	$()
	@mkdir -p $(HOME)/.minikube/
	@rm -f $(HOME)/.minikube/bootstrap.sh
	curl -fL -o $(HOME)/.minikube/bootstrap.sh \
		https://raw.githubusercontent.com/instrumentisto/toolchain/master/minikube/bootstrap.sh
	@chmod +x $(HOME)/.minikube/bootstrap.sh
endef




##################
# .PHONY section #
##################

.PHONY: build build.jason \
        cargo cargo.build.jason cargo.changelog.link cargo.fmt cargo.gen \
        	cargo.lint cargo.version \
        docker.build \
        	docker.down.control docker.down.coturn docker.down.demo \
        	docker.down.e2e docker.down.medea docker.down.webdriver  \
        	docker.pull docker.push docker.tag docker.tar docker.untar \
        	docker.up.control docker.up.coturn docker.up.demo docker.up.e2e \
        	docker.up.medea docker.up.webdriver \
        docs docs.rust \
        down down.control down.coturn down.demo down.dev down.medea \
        flutter flutter.fmt flutter.lint flutter.run \
        	flutter.android.compile_api_version \
        	flutter.android.min_api_version \
        	flutter.web.assets flutter.gen \
        helm helm.dir helm.down helm.lint helm.list \
        	helm.package helm.package.release helm.up \
        minikube.boot \
        release release.crates release.helm release.npm \
        rustup.targets \
        test test.e2e test.flutter test.unit \
        up up.control up.coturn up.demo up.dev up.jason up.medea \
        wait.port \
        yarn yarn.version
