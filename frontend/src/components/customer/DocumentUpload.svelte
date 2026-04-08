<script lang="ts">
  interface UploadedDoc {
    name: string;
    type: string;
    preview?: string;
  }

  interface Props {
    documents: UploadedDoc[];
    onnext: () => void;
    onprev: () => void;
  }

  let { documents = $bindable(), onnext, onprev }: Props = $props();
  let dragActive = $state(false);

  function handleFileChange(e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files) return;
    processFiles(input.files);
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    dragActive = false;
    if (e.dataTransfer?.files) {
      processFiles(e.dataTransfer.files);
    }
  }

  function processFiles(files: FileList) {
    for (const file of Array.from(files)) {
      const reader = new FileReader();
      reader.onload = () => {
        documents = [
          ...documents,
          {
            name: file.name,
            type: file.type,
            preview: file.type.startsWith('image/') ? (reader.result as string) : undefined,
          },
        ];
      };
      reader.readAsDataURL(file);
    }
  }

  function removeDoc(index: number) {
    documents = documents.filter((_, i) => i !== index);
  }
</script>

<div class="space-y-4">
  <h3 class="text-lg font-medium text-gray-900">
    Documents justificatifs
  </h3>

  <!-- Drop zone -->
  <div
    class="rounded-lg border-2 border-dashed p-8 text-center transition-colors
      {dragActive ? 'border-blue-500 bg-blue-50' : 'border-gray-300 bg-gray-50'}"
    role="region"
    aria-label="Zone de depot de fichiers"
    ondragover={(e) => { e.preventDefault(); dragActive = true; }}
    ondragleave={() => (dragActive = false)}
    ondrop={handleDrop}
  >
    <svg class="mx-auto mb-3 h-10 w-10 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
    </svg>
    <p class="mb-2 text-sm text-gray-600">
      Glissez-deposez vos fichiers ici ou
    </p>
    <label
      for="doc-upload"
      class="inline-block cursor-pointer rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-within:outline-2 focus-within:outline-offset-2 focus-within:outline-blue-600"
    >
      Parcourir
      <input
        id="doc-upload"
        type="file"
        multiple
        accept="image/*,.pdf"
        class="sr-only"
        onchange={handleFileChange}
        aria-label="Telecharger un document"
        data-testid="kyc-document-input"
      />
    </label>
    <p class="mt-2 text-xs text-gray-500">PDF, JPG, PNG (max 10 Mo)</p>
  </div>

  <!-- Uploaded files -->
  {#if documents.length > 0}
    <ul class="space-y-2" aria-label="Documents telecharges">
      {#each documents as doc, i}
        <li class="flex items-center gap-3 rounded-md border border-gray-200 bg-white p-3">
          {#if doc.preview}
            <img src={doc.preview} alt="Apercu de {doc.name}" class="h-12 w-12 rounded object-cover" />
          {:else}
            <div class="flex h-12 w-12 items-center justify-center rounded bg-gray-100 text-xs text-gray-500" aria-hidden="true">
              PDF
            </div>
          {/if}
          <span class="flex-1 truncate text-sm text-gray-700">{doc.name}</span>
          <button
            type="button"
            onclick={() => removeDoc(i)}
            class="text-sm text-red-600 hover:text-red-800 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600"
            aria-label="Supprimer {doc.name}"
          >
            Supprimer
          </button>
        </li>
      {/each}
    </ul>
  {/if}

  <div class="flex justify-between pt-4">
    <button
      type="button"
      onclick={onprev}
      class="rounded-md border border-gray-300 bg-white px-6 py-2 text-sm font-semibold text-gray-700 shadow-sm hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
    >
      Precedent
    </button>
    <button
      type="button"
      onclick={onnext}
      class="rounded-md bg-blue-600 px-6 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
    >
      Suivant
    </button>
  </div>
</div>
