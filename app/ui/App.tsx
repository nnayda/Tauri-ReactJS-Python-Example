
export default function App() {
    function ping() {
        
        fetch("http://127.0.0.1:51236/")
            .then((response) => response.json())
            .then((data) => {
                console.log(data);
                let response_area = document.getElementById("response-area");
                let new_child = document.createElement("p");
                new_child.innerText = data['response'];
                response_area?.appendChild(new_child);
            })
            .catch(err =>{
                console.log(err);
            });
    }

    return (
        <div>
            <button onClick={ping}>Ping</button>
                <div id="response-area">
            </div>
        </div>
    )
}