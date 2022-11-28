#!/usr/bin/env bb
(ns user
  (:require [babashka.fs :as fs]
            [clojure.string :as str]))

(def allowed-files
  #{"envrc" "shell.nix" "justfile" "Makefile"})

(def mappings
  {"envrc" ".envrc"})

(defn map-file [f]
  (let [f-name (fs/file-name f)]
    (mappings f-name f-name)))

(def metadata-repo "~/code/scratch/nix-shells/")

(defn confirm [text]
  (println (format "%s [y/N] " text))
  (let [ans (str/trim (read-line))]
    (= "y" ans)))

(defn link-dir
  [args]
  (let [target-dir (or args ".") ;; usually run from the repo we want to add shell.nix to
        metadata-dir (fs/path (fs/expand-home metadata-repo) (-> target-dir fs/canonicalize fs/file-name))]
    (if-not (fs/exists? metadata-dir)
      (do (println "No metadata exists for this dir")
          (if (confirm (format "Create new metadata dir %s?" metadata-dir))
            (do (fs/create-dir metadata-dir)
                (println "Created dir"))
            (System/exit 1)))
      (let [files (fs/list-dir metadata-dir)
            filtered (filter (fn [f] (contains? allowed-files (fs/file-name f))) files)
            targets (for [f filtered]
                      [(fs/canonicalize f) (fs/path target-dir (map-file f))])]
        (println (format "Will link files: %s to %s " (pr-str (map str filtered)) target-dir))
        (doseq [[path target] targets]
          (if (fs/exists? target)
            (println "Already exists: " (str target))
            (do
              (println (format "will create symlink from %s to %s " path target))
              (fs/create-sym-link target path))))))))

(when (= *file* (System/getProperty "babashka.file"))
  (link-dir *command-line-args*))

(comment
  (prn (link-dir "./lima")))
