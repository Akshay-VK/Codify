<script>
  import { resolveResource } from '@tauri-apps/api/path'
  // alternatively, use `window.__TAURI__.path.resolveResource`
  import { readTextFile } from '@tauri-apps/api/fs'
  // alternatively, use `window.__TAURI__.fs.readTextFile`

  async function getConfigData(){
    // `lang/de.json` is the value specified on `tauri.conf.json > tauri > bundle > resources`
    const resourcePath = await resolveResource('lang/de.json')
    const langDe = JSON.parse(await readTextFile(resourcePath))

    console.log(langDe.hello) // This will print 'Guten Tag!' to the devtools console
  }
</script>

<h1 class="text-3xl font-bold underline">
  Hello world!
</h1>