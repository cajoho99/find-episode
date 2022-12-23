import { QueryClient, QueryClientProvider, useQuery, useQueryClient } from '@tanstack/react-query'
import { useState } from 'react'

interface TranscriptEntry {
    text: string,
    start: number,
    duration: number,
}

interface Video {
    id: string,
    thumbnail: string,
    title: string,
    transcription: TranscriptEntry[],
}

function App() {
    const [search, setSearch] = useState("")
    const queryClient = useQueryClient()
    return (
        <div className='flex flex-col items-center'>
            <h1 className="text-4xl text-bold m-8">Find Episode</h1>
            <input
                type="text"
                placeholder="Type here"
                className="input input-bordered input-primary w-full max-w-xs mb-8"
                onChange={e => {
                    setSearch(e.target.value)
                }} />
            <SearchResults search={search} />
        </div>
    )
}


const SearchResults: React.FC<{ search: string }> = ({ search }) => {
    const { isLoading, isError, data, error, isRefetching} = useQuery({
        queryKey: ['search', search],
        queryFn: () =>
            fetch('http://localhost:8000/search/' + search).then(res =>
                res.json()
            )
        ,
        select: data => data as Video[],
        refetchOnMount: "always"
    })
    


    if (isLoading) {
        return <span>Loading...</span>
    }

    if (isError) {
        return <div>Error!!!</div>
    }

    if (isRefetching) {
        return <div>Getting results...</div>
    }

    return (<div className='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8'>
        {data.map(d => VideoResult(d, search))}
    </div>)
}


function VideoResult(video: Video, search: string) {
    return (
        <div key={video.id} className="card w-96 bg-base-100 shadow-xl">
            <figure><img src={video.thumbnail} alt="Thumbnail of video" /></figure>
            <div className="card-body">
                <h2 className="card-title">{video.title}</h2>
                <ul>
                    {video.transcription.map(t => BoldSearchedWord(search, t.text))}
                </ul>
            </div>
        </div>
    )
}


const BoldSearchedWord = (search: string, text: string) => {

    const re = new RegExp(search, "g");
    let newText = text.replace(re, "<b>" + search + "</b>");

    return (
        <li
            dangerouslySetInnerHTML={{ __html: '<i>"...' + newText + '..."</i>' }}
        />
    )
}

export default App
