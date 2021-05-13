.PHONY: frontend
frontend:
	@cd frontend && yarn serve

backend:
	@cd src-tauri && yarn tauri dev

frontend/dist:
	cd frontend && yarn build



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