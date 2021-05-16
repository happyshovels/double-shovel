.PHONY: frontend
frontend:
	@cd frontend && yarn serve

backend:
	@cd src-tauri && yarn tauri dev


release: frontend/dist
	rm -rf frontend/dist
	cd frontend && yarn build
	cd src-tauri && yarn tauri build
	echo ""

run:
	@./src-tauri/target/debug/double-shovel

.PHONY: clean release 
clean: 
	echo "todo clean"
	
clean_frontend: 
	rm -rf frontend/dist