<script lang="ts">
  import { onMount } from 'svelte';
  import { requestApi } from '../api';
  import type { FileEntry, FileContent } from '#/types';
  import { ChevronRight, ChevronDown, File, Folder, Eye, Download } from 'lucide-svelte';
  import { highlight, languages } from 'prismjs';
  import 'prismjs/components/prism-typescript';
  import 'prismjs/components/prism-javascript';
  import 'prismjs/components/prism-json';
  import 'prismjs/components/prism-css';
  import 'prismjs/components/prism-scss';
  import 'prismjs/components/prism-markdown';
  import 'prismjs/components/prism-yaml';
  import 'prismjs/components/prism-rust';
  import 'prismjs/components/prism-toml';
  import 'prismjs/themes/prism-tomorrow.css';

  export let packageName: string;
  export let packageVersion: string;

  let files: FileEntry[] = [];
  let selectedFile: string | null = null;
  let fileContent: FileContent | null = null;
  let loading = false;
  let expandedDirs = new Set<string>();

  onMount(async () => {
    await loadFiles();
  });

  async function loadFiles() {
    loading = true;
    try {
      const response = await requestApi<{ success: boolean; data: FileEntry[] }>('GET', `/api/packages/${encodeURIComponent(packageName)}/${encodeURIComponent(packageVersion)}/files`);
      files = response.data || [];
      
      // Auto-select the first readable file
      if (files.length > 0) {
        const firstFile = findFirstReadableFile(files);
        if (firstFile) {
          await loadFileContent(firstFile);
        }
      }
    } catch (error) {
      console.error('Failed to load files:', error);
    } finally {
      loading = false;
    }
  }

  function findFirstReadableFile(fileList: FileEntry[]): string | null {
    for (const file of fileList) {
      if (file.type === 'file') {
        // Prefer common files like README, index files, etc.
        const fileName = file.name.toLowerCase();
        if (fileName.includes('readme') || fileName.includes('index') || fileName.endsWith('.md') || fileName.endsWith('.ts') || fileName.endsWith('.js')) {
          return file.path;
        }
      }
    }
    
    // If no preferred file found, return the first file
    for (const file of fileList) {
      if (file.type === 'file') {
        return file.path;
      }
    }
    
    return null;
  }

  async function loadFileContent(filePath: string) {
    if (selectedFile === filePath && fileContent) return;

    loading = true;
    selectedFile = filePath;
    fileContent = null;

    try {
      const response = await requestApi<{ success: boolean; data: FileContent }>('GET', `/api/packages/${encodeURIComponent(packageName)}/${encodeURIComponent(packageVersion)}/file?path=${encodeURIComponent(filePath)}`);
      fileContent = response.data;
    } catch (error) {
      console.error('Failed to load file content:', error);
      fileContent = { content: 'Error loading file content', encoding: 'utf-8', mimeType: 'text/plain' };
    } finally {
      loading = false;
    }
  }

  function toggleDirectory(path: string) {
    if (expandedDirs.has(path)) {
      expandedDirs.delete(path);
    } else {
      expandedDirs.add(path);
    }
    expandedDirs = expandedDirs;
  }

  function getLanguageFromPath(filePath: string): string {
    const ext = filePath.split('.').pop()?.toLowerCase();
    switch (ext) {
      case 'ts': return 'typescript';
      case 'tsx': return 'typescript';
      case 'js': return 'javascript';
      case 'jsx': return 'javascript';
      case 'json': return 'json';
      case 'css': return 'css';
      case 'scss': return 'scss';
      case 'sass': return 'scss';
      case 'md': return 'markdown';
      case 'yml': case 'yaml': return 'yaml';
      case 'rs': return 'rust';
      case 'toml': return 'toml';
      case 'html': return 'html';
      case 'xml': return 'xml';
      case 'svg': return 'xml';
      default: return 'text';
    }
  }

  function highlightCode(content: string, language: string): string {
    if (languages[language]) {
      return highlight(content, languages[language], language);
    }
    return content;
  }

  function formatFileSize(size: number): string {
    if (size < 1024) return `${size} B`;
    if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
    return `${(size / (1024 * 1024)).toFixed(1)} MB`;
  }

  async function downloadFile() {
    if (!selectedFile) return;

    try {
      const response = await fetch(`/api/packages/${encodeURIComponent(packageName)}/${encodeURIComponent(packageVersion)}/file?path=${encodeURIComponent(selectedFile)}`);
      if (!response.ok) throw new Error('Failed to download file');

      const data = await response.json();
      if (!data.success) throw new Error(data.error);

      const fileContent = data.data;
      const blob = fileContent.encoding === 'base64'
        ? new Blob([Uint8Array.from(atob(fileContent.content), c => c.charCodeAt(0))], { type: fileContent.mimeType })
        : new Blob([fileContent.content], { type: fileContent.mimeType || 'text/plain' });

      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = selectedFile.split('/').pop() || 'download';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Failed to download file:', error);
      alert('Failed to download file. Please try again.');
    }
  }

  async function downloadPackage() {
    try {
      const url = `/api/packages/${encodeURIComponent(packageName)}/${encodeURIComponent(packageVersion)}/download`;
      const a = document.createElement('a');
      a.href = url;
      a.download = `${packageName}-${packageVersion}.tar.gz`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
    } catch (error) {
      console.error('Failed to download package:', error);
      alert('Failed to download package. Please try again.');
    }
  }

  function viewFile() {
    if (!selectedFile || !fileContent) return;

    // For text files, create a new window and display the content directly
    if (fileContent.encoding !== 'base64' && fileContent.mimeType.startsWith('text/')) {
      const newWindow = window.open('', '_blank');
      if (newWindow) {
        newWindow.document.write(`
          <!DOCTYPE html>
          <html>
            <head>
              <title>${selectedFile}</title>
              <style>
                body { 
                  font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace; 
                  margin: 20px; 
                  background: #1e1e1e; 
                  color: #d4d4d4; 
                  line-height: 1.5; 
                }
                pre { 
                  white-space: pre-wrap; 
                  word-wrap: break-word; 
                  margin: 0; 
                }
                h1 { 
                  font-size: 16px; 
                  margin-bottom: 20px; 
                  color: #569cd6; 
                  border-bottom: 1px solid #404040; 
                  padding-bottom: 10px; 
                }
              </style>
            </head>
            <body>
              <h1>${selectedFile}</h1>
              <pre>${fileContent.content.replace(/</g, '&lt;').replace(/>/g, '&gt;')}</pre>
            </body>
          </html>
        `);
        newWindow.document.close();
      }
    } else {
      // For binary files or images, use blob URL
      const blob = fileContent.encoding === 'base64'
        ? new Blob([Uint8Array.from(atob(fileContent.content), c => c.charCodeAt(0))], { type: fileContent.mimeType })
        : new Blob([fileContent.content], { type: fileContent.mimeType || 'text/plain' });

      const url = URL.createObjectURL(blob);
      window.open(url, '_blank');

      // Clean up the URL after a delay to allow the browser to open it
      setTimeout(() => URL.revokeObjectURL(url), 1000);
    }
  }

  function renderFileTree(fileList: FileEntry[], depth = 0): FileEntry[] {
    return fileList.sort((a, b) => {
      if (a.type !== b.type) {
        return a.type === 'directory' ? -1 : 1;
      }
      return a.name.localeCompare(b.name);
    });
  }
</script>

<div class="file-browser">
  {#if loading && files.length === 0}
    <div class="loading">
      <div class="spinner"></div>
      <span>Loading files...</span>
    </div>
  {:else if files.length === 0}
    <div class="empty-state">
      <File class="w-8 h-8 text-gray-400" />
      <p class="text-gray-500">No files found in this package</p>
    </div>
  {:else}
    <div class="file-explorer">
      <div class="file-tree">
        <h3 class="tree-header">
          <Folder class="w-4 h-4" />
          Package Files
        </h3>

        {#each renderFileTree(files) as file}
          <div class="file-item" style="margin-left: {file.path.split('/').length - 1}rem">
            {#if file.type === 'directory'}
              <button
                class="dir-toggle"
                on:click={() => toggleDirectory(file.path)}
              >
                {#if expandedDirs.has(file.path)}
                  <ChevronDown class="w-4 h-4" />
                {:else}
                  <ChevronRight class="w-4 h-4" />
                {/if}
                <Folder class="w-4 h-4 text-blue-500" />
                <span class="file-name">{file.name}</span>
              </button>

              {#if expandedDirs.has(file.path) && file.children}
                {#each renderFileTree(file.children) as child}
                  <div class="file-item nested" style="margin-left: {(file.path.split('/').length)}rem">
                    <button
                      class="file-button {selectedFile === child.path ? 'selected' : ''}"
                      on:click={() => loadFileContent(child.path)}
                    >
                      <File class="w-4 h-4 text-gray-500" />
                      <span class="file-name">{child.name}</span>
                      {#if child.size}
                        <span class="file-size">{formatFileSize(child.size)}</span>
                      {/if}
                    </button>
                  </div>
                {/each}
              {/if}
            {:else}
              <button
                class="file-button {selectedFile === file.path ? 'selected' : ''}"
                on:click={() => loadFileContent(file.path)}
              >
                <File class="w-4 h-4 text-gray-500" />
                <span class="file-name">{file.name}</span>
                {#if file.size}
                  <span class="file-size">{formatFileSize(file.size)}</span>
                {/if}
              </button>
            {/if}
          </div>
        {/each}
      </div>

      <div class="file-content">
        {#if selectedFile && fileContent}
          <div class="content-header">
            <div class="file-info">
              <File class="w-4 h-4" />
              <span class="file-path">{selectedFile}</span>
            </div>
            <div class="content-actions">
              <button
                class="inline-flex items-center px-3 py-1 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 transition-colors"
                on:click={viewFile}
                title="Open file in new tab"
              >
                <Eye class="w-4 h-4 mr-1" />
                View
              </button>
              <button
                class="inline-flex items-center px-3 py-1 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 transition-colors"
                on:click={downloadFile}
                title="Download this file"
              >
                <Download class="w-4 h-4 mr-1" />
                Download
              </button>
              <button
                class="inline-flex items-center px-3 py-1 border border-blue-500 text-sm font-medium rounded-md text-blue-700 bg-blue-50 hover:bg-blue-100 transition-colors"
                on:click={downloadPackage}
                title="Download entire package"
              >
                <Download class="w-4 h-4 mr-1" />
                Download Package
              </button>
            </div>
          </div>

          <div class="content-body">
            {#if fileContent.encoding === 'base64'}
              <div class="binary-file">
                <div class="binary-info">
                  <File class="w-8 h-8 text-gray-400" />
                  <p>Binary file ({fileContent.mimeType})</p>
                  <p class="text-sm text-gray-500">Cannot display binary content</p>
                </div>
              </div>
            {:else if fileContent.mimeType.startsWith('image/')}
              <div class="image-file">
                <img
                  src="data:{fileContent.mimeType};base64,{fileContent.content}"
                  alt={selectedFile}
                  class="max-w-full h-auto"
                />
              </div>
            {:else}
              <pre class="code-content"><code class="language-{getLanguageFromPath(selectedFile)}">{@html highlightCode(fileContent.content, getLanguageFromPath(selectedFile))}</code></pre>
            {/if}
          </div>
        {:else if selectedFile}
          <div class="loading-content">
            <div class="spinner"></div>
            <span>Loading file content...</span>
          </div>
        {:else}
          <div class="no-selection">
            <Eye class="w-8 h-8 text-gray-400" />
            <p class="text-gray-500">Select a file to view its content</p>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .file-browser {
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    overflow: hidden;
    background: white;
    height: 600px;
  }

  .loading, .empty-state, .no-selection, .loading-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 12px;
  }

  .binary-info {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 2rem;
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid #f3f4f6;
    border-top: 2px solid #3b82f6;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .file-explorer {
    display: grid;
    grid-template-columns: 300px 1fr;
    height: 100%;
  }

  .file-tree {
    border-right: 1px solid #e5e7eb;
    overflow-y: auto;
    background: #f9fafb;
  }

  .tree-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    font-weight: 600;
    border-bottom: 1px solid #e5e7eb;
    background: white;
    margin: 0;
  }

  .file-item {
    border-bottom: 1px solid #f3f4f6;
  }

  .file-item.nested {
    background: #f3f4f6;
  }

  .dir-toggle, .file-button {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 12px;
    border: none;
    background: none;
    text-align: left;
    cursor: pointer;
    transition: background-color 0.15s;
  }

  .dir-toggle:hover, .file-button:hover {
    background: #e5e7eb;
  }

  .file-button.selected {
    background: #dbeafe;
    border-right: 2px solid #3b82f6;
  }

  .file-name {
    flex: 1;
    font-size: 14px;
    font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
  }

  .file-size {
    font-size: 12px;
    color: #6b7280;
  }

  .file-content {
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .content-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid #e5e7eb;
    background: white;
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .file-path {
    font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
    font-size: 14px;
    color: #374151;
  }

  .content-actions {
    display: flex;
    gap: 8px;
  }

  .content-body {
    flex: 1;
    overflow: auto;
  }

  .code-content {
    margin: 0;
    padding: 16px;
    font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
    font-size: 13px;
    line-height: 1.5;
    background: #1e1e1e;
    color: #d4d4d4;
    overflow: auto;
    height: 100%;
  }

  .code-content code {
    background: none;
    padding: 0;
    font-size: inherit;
    color: inherit;
  }

  .image-file {
    padding: 16px;
    text-align: center;
  }

  .binary-file {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #6b7280;
  }

  /* Prism theme adjustments */
  :global(.token.comment) { color: #6a9955; }
  :global(.token.string) { color: #ce9178; }
  :global(.token.number) { color: #b5cea8; }
  :global(.token.keyword) { color: #569cd6; }
  :global(.token.function) { color: #dcdcaa; }
  :global(.token.class-name) { color: #4ec9b0; }
  :global(.token.property) { color: #9cdcfe; }
  :global(.token.operator) { color: #d4d4d4; }
  :global(.token.punctuation) { color: #d4d4d4; }
</style>
