.PHONY: frontend
frontend:
	@cd frontend && yarn serve

backend:
	@cd src-tauri && yarn tauri dev

frontend/dist:
	cd frontend && yarn build
	cp frontend/src/splash.html frontend/dist/splash.html
	cp frontend/src/app-icon.png frontend/dist/app-icon.png

frontend/dist/splash.html frontend/dist/app-icon.png: frontend/dist
	cp frontend/src/$(notdir $@) $@

release_frontend: frontend/dist frontend/dist/splash.html frontend/dist/app-icon.png
	@echo "built frontend for release"

src-tauri/target/release:
	cd src-tauri/target/release

release: frontend/dist
	cd src-tauri/target/release && yarn tauri build
	echo ""

#launch_release:
#	open 

.PHONY: clean clean_frontend release 
clean: clean_frontend
	echo "clean done."
	
clean_frontend: 
	rm -rf frontend/dist