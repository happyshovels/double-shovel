.PHONY: frontend
frontend:
	@cd frontend && yarn serve

backend:
	@cd src-tauri && yarn tauri dev

frontend/dist:
	cd frontend && yarn build


release_frontend: frontend/dist
	@echo "built frontend for release"

src-tauri/target/release:
	cd src-tauri/target/release

release: frontend/dist
	cd src-tauri && yarn tauri build
	echo ""

run:
	@./src-tauri/target/debug/double-shovel

.PHONY: clean clean_frontend release 
clean: clean_frontend
	echo "clean done."
	
clean_frontend: 
	rm -rf frontend/dist